import { CronCapability, handler, Runner, type Runtime } from "@chainlink/cre-sdk";

import { applyRules } from "./steps/rules.js";
import { chainState } from "./steps/chain.js";
import { prices } from "./steps/prices.js";
import { buildRemainingAccounts, submitReport } from "./steps/report.js";
import { screen } from "./steps/screen.js";

export type WorkflowConfig = {
  schedule: string;
  cluster: "devnet" | "mainnet";
  chainSelector: string;
  programId: string;
  forwarderProgramId: string;
  forwarderState: string;
  screeningSource: "fixture" | "live";
  useMockRoute: boolean;
};

const onCronTrigger = (runtime: Runtime<WorkflowConfig>): string => {
  const config = runtime.config;
  runtime.log(`amanah-screen skeleton cluster=${config.cluster} source=${config.screeningSource}`);
  const screened = screen(runtime);
  const quoted = prices(runtime);
  const chain = chainState(runtime);
  const rules = applyRules(screened);
  const accounts = buildRemainingAccounts(runtime);
  const write = submitReport(runtime, rules);
  runtime.log(
    `stub rows=${screened.length} prices=${quoted.length} universe=${chain.universeVersion} accounts=${accounts.length} written=${write.written}`,
  );
  return "stub";
};

export const initWorkflow = (config: WorkflowConfig) => {
  const cron = new CronCapability();
  return [handler(cron.trigger({ schedule: config.schedule }), onCronTrigger)];
};

export async function main(): Promise<void> {
  const runner = await Runner.newRunner<WorkflowConfig>();
  await runner.run(initWorkflow);
}
