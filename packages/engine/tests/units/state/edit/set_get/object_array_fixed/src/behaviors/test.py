def behavior(state, context):
    """Gets and sets a struct fixed-size-array using set/get notation"""
    state["o1_is_list"] = type(state["o1"]) is list
    state["o1_0_is_struct"] = type(state["o1"][0]) is dict
    state["o1_0_n1_is_number"] = type(state["o1"][0]["n1"]) is float
    state["o1_1_n1_is_number"] = type(state["o1"][1]["n1"]) is float

    o1_0 = {"n1": state.get("o1")[0]['n1'] * 10, "n2": state.get("o1")[0]["n1"] * 5}
    o1_1 = {"n1": state.get("o1")[1]["n1"] * 20, "n2": state.get("o1")[1]["n1"] * 10}
    state.set("o1", [o1_0, o1_1])

    state["o1_0_n2_is_number"] = type(state["o1"][0]["n2"]) is float
    state["o1_1_is_struct"] = type(state["o1"][0]) is dict
    state["o1_1_n2_is_number"] = type(state["o1"][1]["n2"]) is float
