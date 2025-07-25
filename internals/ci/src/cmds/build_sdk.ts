import { spawnSync } from "node:child_process";

import { paths } from "../paths.ts";

export async function buildSDK(...args: any[]) {
  console.log("buildSDK, args: %j", args);

  // spawnSync("yarn", ["run", "build"], {
  //   cwd: paths.cait_sith_addon_addon,
  //   stdio: "inherit",
  // });
  //
  // spawnSync("yarn", ["run", "build:wasm"], {
  //   cwd: paths.cait_sith_keplr_wasm,
  //   stdio: "inherit",
  // });
  //
  // spawnSync("yarn", ["run", "copy:wasm"], {
  //   cwd: paths.ewallet_attached,
  //   stdio: "inherit",
  // });
}
