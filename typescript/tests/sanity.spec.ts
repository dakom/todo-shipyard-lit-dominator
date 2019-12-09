const fs = require('fs');

import { BridgeEvent } from "@events/events";
import { get_enum_pairs, enum_pairs_to_list } from "@utils/enums";
const {get_bridge_event_pairs} = require("../../_static/wasm/core/pkg/my_core");

const rust_bridge_events = enum_pairs_to_list(get_bridge_event_pairs());
const ts_bridge_events = enum_pairs_to_list(get_enum_pairs(BridgeEvent));

describe("all ts types have rust equivilent", () => {
    ts_bridge_events.forEach((name, index) => {
        test(`[${name}] exists in rust bridge event`, () => {
           expect(rust_bridge_events[index]).toBe(name)
        });
    });
});

describe("all rust types have ts equivilent", () => {
    rust_bridge_events.forEach((name, index) => {
        test(`[${name}] exists in ts bridge event`, () => {
           expect(ts_bridge_events[index]).toBe(name)
        });
    });
});