import init, { run_app } from "./pkg/thoth_ui.js";

async function main() {
  await init("/admin/thoth_ui_bg.wasm");
  run_app();
}

main();
