const json_deepcopy = hash_util.json_deepcopy;

/// Neighbor getters (`neighbor.your_state_field`)
const gen_state_getters = (Neighbor, agent_schema) => {
  // Don't need to deepcopy in these getters, because everything in
  // context pools should already be (deep (i.e. recursively)) frozen.
  for (var i = 0; i < agent_schema.fields.length; ++i) {
    const name = agent_schema.fields[i].name;
    const getter =
      name === "agent_id"
        ? function () {
            // TODO: Run `uuid_to_str` on entire `agent_id` column when loading arrow?
            //       Cache loaded `agent_id`? (e.g. `this.__id`)
            const l = this.__loc;
            return hash_util.uuid_to_str(
              this.__snapshot.agent_pool[l.get(0)].cols.agent_id[l.get(1)],
            );
          }
        : function () {
            const l = this.__loc;
            const col = this.__snapshot.agent_pool[l.get(0)].cols[name];
            if (!col) {
              throw new ReferenceError("Missing neighbor field: " + name);
            }
            return col[l.get(1)];
          };
    Object.defineProperty(Neighbor.prototype, name, { get: getter });
  }

  /// Returns neighbor's old outbox (i.e. outbox at the end of the previous step).
  const msgs_getter = function () {
    const p = this.__prev_loc;
    return this.__snapshot.message_pool[p.get(0)].cols.messages[p.get(1)];
  };
  Object.defineProperty(Neighbor.prototype, "messages", { get: msgs_getter });
};

/// `neighbor.to_json()`
const gen_to_json = (agent_schema) => {
  return function () {
    const json_neighbor = {};
    for (var i_field = 0; i_field < agent_schema.fields.length; ++i_field) {
      const name = agent_schema.fields[i_field].name;
      json_neighbor[name] = json_deepcopy(this[name]);
    }
    return json_neighbor;
  };
};

/// AgentContext getter (`agent_context.neighbors`)
const gen_neighbor_getter = (Neighbor) => {
  return (agent_context, elem) => {
    const neighbors = [];
    const snapshot = agent_context.state_snapshot;
    for (var i_neighbor = 0; i_neighbor < elem.length; ++i_neighbor) {
      neighbors[i_neighbor] = new Neighbor(
        snapshot,
        agent_context.__prev_loc,
        elem.get(i_neighbor),
      );
    }
    return neighbors;
  };
};

const gen_neighbor = (agent_schema) => {
  const Neighbor = function (state_snapshot, prev_loc, loc) {
    this.__snapshot = state_snapshot;
    this.__prev_loc = prev_loc; // For looking up messages in snapshot message pool
    this.__loc = loc; // For looking up neighbor agent fields in snapshot agent pool
  };
  gen_state_getters(Neighbor, agent_schema);
  Neighbor.prototype.to_json = gen_to_json(agent_schema);
  return Object.freeze(Neighbor);
};

const loaders = {
  /// Context batch `neighbors` column loader
  /// (The `neighbors` column is visible only via the getter above.)
  neighbors: hash_util.load_shallow,

  // `__prev_loc` isn't meant to be visible to package users at all,
  // and doesn't need a custom loader due to the double underscores.
};

const start_sim = (_experiment, _sim, _init_message, init_context) => {
  const Neighbor = gen_neighbor(init_context.agent_schema);
  const getters = {
    neighbors: gen_neighbor_getter(Neighbor),
  };
  return {
    loaders: loaders,
    getters: getters,
  };
};
