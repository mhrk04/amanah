"use client";

import { CLUSTER_CONFIG, DEFAULT_CLUSTER, resolveCluster } from "@amanah/common";
import { ConnectionProvider, WalletProvider } from "@solana/wallet-adapter-react";
import { PhantomWalletAdapter, SolflareWalletAdapter } from "@solana/wallet-adapter-wallets";
import { useMemo, type ReactNode } from "react";

/**
 * TODO(T20): the wallet connection should talk to `/api/rpc`, which forwards
 * to the cluster RPC. Keys stay on the server.
 */
export function Providers({ children }: { children: ReactNode }) {
  const cluster = resolveCluster(process.env.NEXT_PUBLIC_CLUSTER ?? DEFAULT_CLUSTER);
  const endpoint = process.env.NEXT_PUBLIC_SOLANA_RPC_URL ?? "http://127.0.0.1:3000/api/rpc";
  const wallets = useMemo(() => [new PhantomWalletAdapter(), new SolflareWalletAdapter()], []);
  void CLUSTER_CONFIG[cluster];

  return (
    <ConnectionProvider endpoint={endpoint}>
      <WalletProvider wallets={wallets} autoConnect={false}>
        {children}
      </WalletProvider>
    </ConnectionProvider>
  );
}
