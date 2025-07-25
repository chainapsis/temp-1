import { program } from "commander";

import { typeCheck } from "./cmds/typecheck.ts";
import { buildSDK } from "./cmds/build_sdk.ts";

async function main() {
  const command = program.version("0.0.1").description("EWallet Public CI");

  command.command("typecheck").action(typeCheck);

  command.command("build_sdk").action(buildSDK);

  program.parse(process.argv);
}

main().then();
