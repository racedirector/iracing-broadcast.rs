import init, { build_chat_macro, build_pit_command } from "iracing-broadcast-wasm";

async function run() {
  await init();

  const pitFuel = build_pit_command("fuel", 18);
  const macro = build_chat_macro(3);

  // Send across your bridge contract.
  window.postMessage({ type: "iracing-broadcast", envelope: pitFuel });
  window.postMessage({ type: "iracing-broadcast", envelope: macro });
}

run();
