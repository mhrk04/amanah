import type { ReactNode } from "react";

import { Providers } from "./providers";
import "./globals.css";

export const metadata = {
  title: "Amanah",
  description: "Shariah screening for tokenized stocks, enforced on Solana.",
};

export default function RootLayout({ children }: { children: ReactNode }) {
  return (
    <html lang="en">
      <body>
        <p className="banner">
          Screening results are not a fatwa. Amanah is unaudited hackathon code and is not Shariah
          certification.
        </p>
        <Providers>
          <main>
            <nav>
              <a href="/">Home</a>
              <a href="/start">Start</a>
              <a href="/vault">Vault</a>
              <a href="/log">Log</a>
              <a href="/purify">Purify</a>
            </nav>
            {children}
          </main>
        </Providers>
      </body>
    </html>
  );
}
