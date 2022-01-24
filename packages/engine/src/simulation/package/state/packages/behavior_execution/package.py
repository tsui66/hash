import sys
import traceback


def _hash_behavior_id(lang_index, id_within_lang):
    # TODO: Either keep in sync with behavior id generation on Rust side of
    #       behavior execution package or change Rust side to generate
    #       single numbers (rather than pairs of numbers) in the first place.

    # We support fewer than 10 languages.
    return id_within_lang*10 + lang_index


# `behavior_descs` should be a list of objects that have fields `id`, `name`, `source`, `columns`,
# `language` and `dyn_access`.
def _load_behaviors(behavior_descs):
    behaviors = {}
    warnings = []  # TODO: Accumulate warnings automatically with something like `hash_util.warn`
    for desc in behavior_descs:
        if desc['language'] != "Python":
            behaviors[_hash_behavior_id(desc['id'][0], desc['id'][1])] = {
                "name": desc['name'],
                "language": desc['language'],
            }
            continue

        try:
            # behavior_globals should contain a callable `behavior` if the user's code is correct
            behavior_globals = {}
            bytecode = compile(desc['source'], desc['name'], "exec")
            exec(bytecode, behavior_globals)
            behavior_fn = behavior_globals.get("behavior")
            
            if callable(behavior_fn):
                behaviors[_hash_behavior_id(desc['id'][0], desc['id'][1])] = {
                    "name": desc['name'],
                    "language": desc['language'],
                    "required_col_names": None,  # TODO: Propagate desc['columns'] here.
                    "dyn_access": desc['dyn_access'],
                    "fn": behavior_fn
                }
            else:
                warnings.append(
                    f"Couldn't load behavior: No function named 'behavior': {desc['name']}"
                )

        except Exception:
            # Have to catch generic `Exception`, because user's code could throw anything.
            n_pkg_fns = 2
            e = str(traceback.format_exception(*sys.exc_info())[n_pkg_fns:])
            warnings.append(f"Couldn't load behavior `{desc['name']}`: {e}")

        # With the current implementation, failing to load a behavior
        # isn't an error if the behavior is never actually used. This
        # is consistent with Python usually not giving errors until an
        # erroneous piece of code is actually run.
        # TODO: Discuss whether making it an error would help user's debugging.

    return behaviors, warnings


def start_experiment(experiment, init_message, _experiment_context):
    experiment['behaviors'], warnings = _load_behaviors(init_message)
    return {
        "warnings": warnings
    }


def _format_behavior_error(behavior_name, exc_info):
    n_pkg_fns = 2
    return f"Behavior `{behavior_name}` error: {traceback.format_exception(*exc_info)[n_pkg_fns:]}"


def _postprocess(agent_state):
    msgs = agent_state.messages
    for m in msgs:
        # Types are checked here because flush might not happen until
        # several Python behaviors later (due to behavior chaining)
        # and if type becomes correct in the meantime, the error won't
        # be raised at all, so whether an error is raised would depend
        # on the rest of the behavior chain, not just the current behavior.
        # TODO: OPTIM But maybe delay type-checking until flush after all
        #       if it has a big performance impact, since it wouldn't affect
        #       correct user code and isn't very likely to affect incorrect
        #       code.

        if isinstance(m["to"], str):
            m["to"] = [m["to"]]
        elif type(m["to"]) != list:
            raise TypeError(f"Message `to` field must be a list or a string: {m}")

        if not isinstance(m["type"], str):
            raise TypeError(f"Message `type` field must be a string: {m}")

    # TODO: OPTIM hasattr outside of loop over agents might be better than
    #       this try/except inside the loop.
    try:
        position = agent_state.position
        if position is not None:  # TODO: OPTIM Is position nullable?
            while len(position) < 3:
                position.append(0.0)
    except AttributeError:
        pass

    # TODO: OPTIM Same todos as for position
    try:
        direction = agent_state.direction
        if direction is not None:
            while len(direction) < 3:
                direction.append(0.0)
    except AttributeError:
        pass


# For each agent in the given group, execute behaviors and postprocess state,
# starting after the last behavior already executed (during this step / more generally
# behavior execution package call) and stopping when all behaviors are executed or
# the next behavior is in a different language (i.e. not Python).
def run_task(experiment, _sim, _task_message, group_state, group_context):
    next_lang = None
    agent_state = None
    agent_context = None

    # TODO: Propagate field specs to runners and use in state and context objects
    behavior_ids_field_key = '_PRIVATE_14_behavior_ids'

    for i_agent in range(group_state.n_agents()):
        # Reuse `agent_state` and `agent_context` objects.
        agent_state = group_state.get_agent(i_agent, agent_state)
        agent_context = group_context.get_agent(i_agent, agent_context)

        # ids of behaviors of this agent
        behavior_ids = getattr(agent_state, behavior_ids_field_key)

        # `behavior_index` is the index of the first behavior that
        # hasn't been executed yet (during this step / package call).
        for i_behavior in range(int(agent_state.behavior_index), len(behavior_ids)):
            agent_state.behavior_index = i_behavior  # Keep up to date for use in behaviors.

            # Behavior ids are shallow-loaded as an optimization, so
            # need to convert to a Python object here.
            # TODO: OPTIM Use `.values` attribute after upgrading Arrow.
            b_id = behavior_ids[i_behavior].as_py()
            behavior = experiment['behaviors'][_hash_behavior_id(b_id[0], b_id[1])]
            if behavior['language'] != "Python":
                next_lang = behavior['language']  # Multiple assignments are fine.
                break

            agent_state.set_dynamic_access(behavior['dyn_access'])
            try:
                behavior['fn'](agent_state, agent_context)
                _postprocess(agent_state)
                
            except Exception:
                # Have to catch generic `Exception`, because user's code could throw anything.
                
                error = _format_behavior_error(behavior['name'], sys.exc_info())
                agent_state.behavior_index = i_behavior
                return {
                    "target": "Main",
                    "errors": [error]
                }

    return {
        "target": next_lang if next_lang is not None else "Main"
    }
