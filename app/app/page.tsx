import { CLUSTER_CONFIG, DEFAULT_CLUSTER, resolveCluster, VAULT_ERRORS } from "@amanah/common";

export default function HomePage() {
  const cluster = resolveCluster(process.env.CLUSTER ?? DEFAULT_CLUSTER);
  const config = CLUSTER_CONFIG[cluster];

  return (
    <>
      <h1>Amanah</h1>
      <p>
        Shariah screening for tokenized stocks, enforced on-chain. The list below will show mints
        approved under each methodology once a compliance report is on the cluster.
      </p>
      <p>
        Cluster: <strong>{config.cluster}</strong>. Jupiter: {config.useJupiter ? "yes" : "no"}.
        Pyth: {config.usePyth ? "yes" : "no"}. Mock route: {config.useMockRoute ? "yes" : "no"}.
      </p>
      <p>TODO(T20): live approved list per methodology, latest report id, age, and Solscan link. No wallet required.</p>
      <p>MintNotApproved is error {VAULT_ERRORS.MintNotApproved} in the design order. Confirm from the IDL in T04.</p>
    </>
  );
}
