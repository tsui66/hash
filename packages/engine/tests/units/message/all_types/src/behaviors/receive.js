/**
 * Reads messages and stores them into the agent's state
 */
const behavior = (state, context) => {
  const messages = context.messages();

  if (messages.length > 0) {
    const data = messages[0].data;

    state.number = data.number;
    state.string = data.string;
    state.bool = data.bool;
    state.struct = data.struct;
    state.number_array = data.number_array;
    state.bool_array = data.bool_array;
    state.struct_array = data.struct_array;
  }
};
