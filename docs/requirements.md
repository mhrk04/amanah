# Amanah: requirements (rev 3)

*Wed 7 Oct 2026, ~01:00 SGT. Pairs with `design.md` (how) and `../amanah-build-tasks.md` (when, who). Priorities: **Must** = the demo or a track fails without it · **Should** = clearly better score, cut only after the cut line · **Could** = first to cut. "Slide" = deck v3 position. Task ids = build plan rev 3.*

Legend: ✅ verified · ⚠️ unverified (see design §13).

---

## 1. Context and constraints

- **Event:** TOKEN2049 Origins hackathon, Singapore. Kickoff Tue 6 Oct 12:00 SGT. **Submission assumed Thu 8 Oct 00:00 SGT** ⚠️ (BuilderBase shows the event ending Thu 8 Oct 17:30 SGT; confirm with organisers in T03). Planning starts Wed 01:15, leaving about **22.75 h**.
- **Team:** 1–2 people (@A = Haziq: Anchor/Rust, keeper; @B = teammate or Haziq-later: CRE, app, content). The plan has a solo schedule too.
- **Tracks (priority order):** Chainlink "Best Workflow with CRE" > "Best Use of Solana" > NOWNodes "Multichain Infrastructure" (gated, see §4.3).
- **Stack (✅ versions):** Anchor 1.2.1 (`anchor-lang`, `anchor-spl`, `@anchor-lang/core`), CRE CLI 1.36.0 + `@chainlink/cre-sdk` 1.23.0 + Bun, Next.js + wallet-adapter, `@solana/kit` 8.4.0 / `@solana/web3.js` 1.99.0, Jupiter `/swap/v2/build`, Pyth Hermes (keyed), Halal Terminal API, NOWNodes `sol.nownodes.io`.
- **Hard rules:** never claim "Shariah-certified" / "halal guaranteed"; screening ≠ fatwa. Don't let an agent run the mainnet CRE `--broadcast` or hold the transmitter key. Proof transactions on **mainnet**.

---

## 2. Functional requirements

### 2.1 On-chain program `amanah_vault`

| ID | Requirement | Pri | Slide | Task |
|---|---|---|---|---|
| FR-01 | `initialize` creates `Config` + `ComplianceRegistry` PDAs with admin, keeper, forwarder program/state, optional transmitter, USDC mint, caps, slippage, max ages | Must | 5 | T04, T05 |
| FR-02 | `set_assets` sets ≤ 8 Token-2022 xStock mints (each must carry ScaledUiAmount), bumps `universe_version`, clears registry approvals, emits `ConfigChanged` | Must | 5 | T05 |
| FR-03 | Admin setters (`set_keeper`, `set_forwarder`, `set_transmitter`, `set_params`, `set_paused`) gated by `has_one = admin`, each emitting `ConfigChanged` | Must | 5 | T05 |
| FR-04 | `open_vault(risk_profile, methodology)` creates `UserVault` PDA `[b"vault", owner]` + pinned vault USDC ATA; `set_profile` changes both later | Must | 5, 6① | T06 |
| FR-05 | `deposit` (USDC, `transfer_checked`) blocked while paused; `withdraw` in kind (USDC or any xStock) by owner only, **allowed while paused** | Must | 6① | T06 |
| FR-06 | `on_report` accepts only a CPI from the configured forwarder: state key + owner, canonical derived authority signer, allowed transmitter (when set), `report_id` strictly increasing, `as_of`/`price_ts` window, `universe_version` match, sorted unique `asset_idx`, reserved bits zero, Σ target ∈ {0, 10000} | Must | 5, 7① | T07, T37 |
| FR-07 | `approved_mask` computed on-chain = `company_pass_mask` if `wrapper_ok && chain_ok`, else 0 | Must | 5, 6② | T07 |
| FR-08 | `pre_swap` enforces: top-level, keeper signer, not paused, no pending, mint approved **for the vault's methodology** (`MintNotApproved`), registry and price fresh, price > 0, per-trade + daily caps, all token accounts pinned, exactly one matching `post_swap`, **only allow-listed programs in between** (`RouteNotAllowed`) | Must | 5, 6③④⑤ | T13 |
| FR-09 | `post_swap` enforces: top-level, pending present, output delta ≥ max(min_out, oracle floor using the **effective** ScaledUiAmount multiplier), fail closed on missing/stale price, keeper ended with ≤ starting USDC | Must | 5, 6③ | T14 |
| FR-10 | Error enum order frozen (MintNotApproved = 6014 with the design order) and printed from the IDL into README | Must | 6⑤, 7③ | T04, T27 |
| FR-11 | Events: `RegistryUpdated`, `SwapGuarded`, `VaultOpened`, `Deposited`, `Withdrawn`, `ConfigChanged`; `ComplianceReportV2` lands in the IDL | Must | 6⑤ | T04, T09 |
| FR-12 | Program deployed to **mainnet**, initialised with the 6-asset universe, keeper, mock forwarder (mainnet) and allowed transmitter | Must | 7 | T08 |
| FR-13 | `security.json` filled (contact, policy, "unaudited hackathon code") | Should | 10 | T28 |
| FR-14 | `initialize` restricted to the program's upgrade authority | Could | n/a | T05 (D5) |

### 2.2 CRE workflow `amanah-screen`

| ID | Requirement | Pri | Slide | Task |
|---|---|---|---|---|
| FR-20 | TS workflow with cron trigger; runs in `cre workflow simulate` (devnet target) and `--target production --broadcast` (mainnet, mock forwarder) | Must | 6②, 7① | T09, T12 |
| FR-21 | Screening via Halal Terminal portfolio scan, normalised to sorted `{sym, mask (5 methodology bits), purif_bps}`, identical consensus; `null` → fail | Must | 5, 6② | T10 |
| FR-22 | Wrapper rule table (STRCx = preferred → `wrapper_ok = false`); chain rule (`chain_ok` = Token-2022, not paused, routable) from a NOWNodes `getMultipleAccounts` read at `finalized` | Must | 5, 6② | T11, T25 |
| FR-23 | Prices: Pyth Hermes (6 xStock feeds, median, `conf/price ≤ 2%`), fallback xStocks quote; `price_ts` = oldest publish time | Must | 5 | T25 |
| FR-24 | Compact Borsh `ComplianceReportV2` (≈153 B for 6 assets, ≤ 263 B budget) written via generated `writeReportFromComplianceReportV2` with `remainingAccounts` [state, authority, registry, config, ix sysvar] | Must | 7① | T11, T12 |
| FR-25 | Determinism: `runtime.now()` only (grep gate: no `Date.now` / `new Date`), integer fields, sorted arrays | Must | n/a | T11 |
| FR-26 | Fail closed: non-200 → throw, no write; zero approved → write the bps-0 "freeze" report; throw unless `txStatus == SUCCESS` | Must | n/a | T11, T12 |
| FR-27 | ≤ 15 HTTP calls per run (design: 3; 8 with the price fallback); passes `--limits default` | Must | n/a | T12 |
| FR-28 | Logs a readable verdict table (asset × methodology, wrapper/chain flags) + Solscan link | Must | 6② | T12 |
| FR-29 | Secrets via `getSecret` (≤ 5): Halal Terminal, Pyth, NOWNodes; fixture mode for dev runs only | Must | n/a | T10 |
| FR-30 | `cre account access` requested; DON deploy run if granted | Should | 10 | T03, T35 |
| FR-31 | Ethereum NVDAx multiplier cross-check via NOWNodes ETH (`eth_call`), +2 HTTP | Could | 9 | T26 |

### 2.3 Keeper

| ID | Requirement | Pri | Slide | Task |
|---|---|---|---|---|
| FR-40 | Plans a buy from the shared deterministic portfolio function for the vault's methodology | Must | 6③ | T17 |
| FR-41 | Builds `[CU limit, CU price] + setup + pre_swap + Jupiter swap + cleanup + post_swap` from `/swap/v2/build` with `destinationTokenAccount = vault ATA`; simulate, CU × 1.2; v0 + ALTs; size fallback (`maxAccounts` step-down, then `transactionVersion=1`) | Must | 6③ | T39 |
| FR-42 | Executes one live guarded mainnet buy (NVDAx, 2–5 USDC) via NOWNodes | Must | 6③, 7② | T18 |
| FR-43 | `--target STRCx` mode sends the same path with `skipPreflight` so the `MintNotApproved` failure lands on-chain | Must | 6④⑤, 7③ | T19 |

### 2.4 Frontend

| ID | Requirement | Pri | Slide | Task |
|---|---|---|---|---|
| FR-50 | `/` live approved list per methodology + latest report (id, age, link), no wallet needed | Must | 6 | T20 |
| FR-51 | `/start` 5-question quiz → profile (deterministic table) + methodology picker (AAOIFI default, DJIM, FTSE, MSCI, S&P) + portfolio preview | Must | 5, 6① | T21 |
| FR-52 | `/vault` wallet connect, open vault + deposit in one tx, holdings at the effective multiplier, withdraw | Must | 6① | T22, T23 |
| FR-53 | `/log` compliance log from on-chain events (reports, guarded swaps) + rejected attempts list | Should | 6⑤ | T24 |
| FR-54 | `/purify` purification card per holding with formula and sources | Should | 8 | T40 |
| FR-55 | LLM explanation of the computed portfolio, labelled "weights are rule-based" | Could | n/a | T41 |
| FR-56 | All RPC via server `/api/rpc` → NOWNodes with method allow-list; no keys in the browser | Must | 9 | T20 |
| FR-57 | Friendly error mapping (6014 → "not approved under your methodology") | Should | 6⑤ | T23 |

### 2.5 Content, purification and submission

| ID | Requirement | Pri | Slide | Task |
|---|---|---|---|---|
| FR-60 | `scripts/purification-check.ts` reproduces $2.49 (AAPLx, $1,000, 12 months, Dividend steps) and computes [N] cents with the Halal Terminal purification rate | Must | 8 | T24 |
| FR-61 | `docs/proof.md` with the 3 mainnet tx links (CRE report, guarded swap, rejected STRCx) + program id + 3 QR codes | Must | 7 | T38 |
| FR-62 | README: elevator pitch, architecture diagram, one section per track with evidence, setup, proof links, limitations (not a fatwa, mock forwarder, permanent delegate, unaudited), business model, roadmap with time frames | Must | all | T27 |
| FR-63 | Deck v3 placeholders filled (names, tx links, program id, [N], team, email, repo, QR) | Must | 1, 7, 8, 11 | T31 |
| FR-64 | Backup video 2:30–3:00, narrated, no music, all 3 proof txs, plays logged out | Must | 6 | T30 |
| FR-65 | Submission form completed (by the user) with repo, video, deck, track selection | Must | n/a | T33 |

---

## 3. Non-functional requirements

### 3.1 Security

| ID | Requirement | Pri |
|---|---|---|
| NFR-S1 | Every token account and mint in `deposit`/`withdraw`/`pre_swap`/`post_swap` derived and constrained (ATA mint + authority + token program; `token_program == mint.owner`) | Must |
| NFR-S2 | Oracle floor fails closed; per-trade cap ≤ 5 USDC and daily cap ≤ 20 USDC during the hackathon | Must |
| NFR-S3 | Report authority hardened (design §5.4 steps 1–7), incl. transmitter allow-list while on the mock forwarder; pause switch | Must |
| NFR-S4 | Instruction introspection: top-level only, forward scan, exact match, uniqueness, route allow-list | Must |
| NFR-S5 | ≥ 15 negative tests pass (design §9.2 test column) before the mainnet deploy of the final build | Must |
| NFR-S6 | Trail of Bits 6-pattern re-review of the real code + `cargo clippy` clean (solana-lints if they build) | Should |
| NFR-S7 | No secrets in git (gitleaks clean); hot wallets ≤ 0.05 SOL; keeper ≠ admin ≠ upgrade authority | Must |
| NFR-S8 | Checked u128 math, floor rounds down, no `unsafe`, no `init_if_needed` on state PDAs | Must |

### 3.2 Latency and performance

| ID | Requirement | Target |
|---|---|---|
| NFR-L1 | CRE simulate run (live sources) end to end | ≤ 60 s (quota 5 min) |
| NFR-L2 | Keeper guarded buy: build → confirmed | ≤ 20 s p50 |
| NFR-L3 | App first paint of the approved list | ≤ 3 s on venue Wi-Fi |
| NFR-L4 | Guarded sandwich tx size | ≤ 1,232 B (v0) or ≤ 4,096 B (v1 fallback) |
| NFR-L5 | Compute: `on_report` ≤ 300k CU limit; sandwich ≤ 1.4M CU | measured in T12/T18 |
| NFR-L6 | Every demo step renders within 8 s or the presenter switches to video | rehearsal |

### 3.3 Cost

| ID | Requirement | Budget |
|---|---|---|
| NFR-C1 | Mainnet SOL total (deploy ×2 buffer, PDAs, ATAs, ~20 txs) | ≤ 3 SOL ⚠️ depends on `.so` size; measure in T08 |
| NFR-C2 | Halal Terminal: ≤ 25 live scans (15 tokens each) over the event | free 500 tokens / 50 requests; Starter $19 if needed |
| NFR-C3 | NOWNodes within the START plan (100K requests/month) | ≈ 2k requests/day |
| NFR-C4 | Demo capital ≤ 20 USDC; trades 2–5 USDC | Must |

### 3.4 Reliability and honesty

| ID | Requirement |
|---|---|
| NFR-R1 | Registry age limit 24 h (demo); CRE broadcast ≤ 1 h before stage |
| NFR-R2 | Proof links open logged-out on Solscan; video plays logged-out |
| NFR-R3 | Wording: "screened under <methodology> using Halal Terminal data", "not a fatwa", "unaudited"; "simulated via Chainlink's mock forwarder" wherever the CRE write is shown; never "certified" |
| NFR-R4 | Every number on a slide has a source in README (multiplier history, purification rate, market figures) |

---

## 4. Track judging map and the evidence each needs

Judging rubrics beyond the track one-liners are **⚠️ unverified** (BuilderBase pages are login-gated). The map uses the one-liners from the kickoff doc, the solana-new `submit-to-hackathon` generic rubric (Functionality 30 / Innovation 25 / Technical execution 20 / Impact 15 / Presentation 10) and the HackathonRadar `submission-reviewer` rubric (5 × 20).

### 4.1 Chainlink: Best Workflow with CRE (primary)

| Likely criterion | Evidence | FR |
|---|---|---|
| CRE is core, not decorative | The registry can **only** change through `on_report`; the guard reads it on every trade; STRCx rejection traces back to a CRE verdict | FR-06, FR-07, FR-08 |
| Uses multiple capabilities | Cron trigger, HTTP (3 sources), node-mode consensus (identical + by-field median), secrets, Solana write | FR-20…29 |
| Uses the new Solana write path well | Generated bindings, compact report sized against the measured forwarder overhead, hardened receiver | FR-24, FR-06 |
| Runs | `cre workflow simulate … --broadcast` terminal recording + mainnet tx (forwarder → `on_report`) | FR-12, FR-61 |
| Production readiness | Determinism gate, fail-closed paths, `--limits default` pass, `cre account access` requested, Keystone switch documented | FR-25, 26, 27, 30 |

### 4.2 Best Use of Solana

| Likely criterion ("a working onchain product") | Evidence | FR |
|---|---|---|
| Working on mainnet | Program id + 3 tx links + live app | FR-12, FR-42, FR-43, FR-61 |
| Uses Solana-native features | Token-2022 ScaledUiAmount read on-chain, PDA vaults, instruction-sysvar sandwich guard, Jupiter atomic composition | FR-08, FR-09 |
| Security | Negative test list, ToB pattern table, threat table in README | NFR-S1…S8 |
| UX | Quiz → deposit in one tx, readable errors, compliance log | FR-51…57 |

### 4.3 NOWNodes: Multichain Infrastructure (gated)

| Criterion ("a working Web3 product using NOWNodes infrastructure") | Evidence | FR |
|---|---|---|
| NOWNodes actually used | Request dashboard screenshot; README table of every call (CRE read, CRE write if T12 works over NOWNodes, keeper, app proxy) | FR-22, FR-42, FR-56 |
| Multichain | Only if T26 ships: NVDAx Ethereum multiplier read via `eth.nownodes.io`, matched to Solana's | FR-31 |

**Gate (design D3):** enter only if (a) the CRE write runs over NOWNodes or (b) T26 ships. Otherwise list NOWNodes under "infrastructure" without entering, to avoid the submission-reviewer "shotgun submission" integrity flag.

### 4.4 Generic rubric self-check (fill in at T36)

| solana-new rubric | Weight | Where we score |
|---|---|---|
| Functionality | 30 | 3 mainnet txs; live app; real STRCx rejection |
| Innovation | 25 | First on-chain *enforced* Shariah screen for tokenized equities; methodology choice enforced per vault; purification from multiplier history |
| Technical execution | 20 | CRE → Solana write; introspection guard; Token-2022 TLV read; ≥ 15 negative tests |
| Potential impact | 15 | Muslim retail investors in MY/SG/ID with xStocks access; open registry for other apps |
| Presentation | 10 | Deck v3, 3-min script, backup video |

| HackathonRadar submission-reviewer (5 × 20) | README must have |
|---|---|
| Problem clarity | Hook question; linked data; Haziq's personal connection |
| Solution story | Logo, name, slogan, elevator pitch up top; team + contributions; one section per judging criterion |
| Tech detail | Stack list + architecture diagram; deep dive on the guard and CRE write; challenges + lessons (forwarder size, Token-2022) |
| Demo quality | Narrated 2:30–3:00 video, no music, working repo/live links |
| Next steps & monetisation | Running costs, 0.5% AUM fee + B2B registry, roadmap (1 month / 6 months / 1 year) |

---

## 5. Acceptance criteria

| ID | Given / when / then | Verifies |
|---|---|---|
| AC-01 | Given the program on mainnet, when the user runs the CRE production broadcast, then a tx shows forwarder → `on_report`, `RegistryUpdated` lists 6 assets with STRCx `wrapper_ok=false`, `approved_mask=0` | FR-06, 07, 20–28 |
| AC-02 | When the keeper buys NVDAx for 2–5 USDC, then one mainnet tx contains `pre_swap` → Jupiter → `post_swap`, `SwapGuarded.out ≥ floor`, and the NVDAx lands in the vault's PDA-owned ATA | FR-08, 09, 41, 42 |
| AC-03 | When the keeper targets STRCx, then the tx fails on-chain with `MintNotApproved` (6014) and the vault holds no STRCx | FR-43 |
| AC-04 | When a second CRE key broadcasts to our receiver, then it fails with `UnknownTransmitter` | FR-06 (T37) |
| AC-05 | The test suite passes ≥ 15 negative cases incl. keeper-own-ATA, low output, stale price, missing/mismatched post_swap, CPI-wrapped pre_swap, route not allowed, fake state, replay, wrong token program | NFR-S5 |
| AC-06 | A new user completes quiz → methodology → open vault + deposit 20 USDC in ≤ 2 minutes with one wallet signature for the vault tx | FR-51, 52 |
| AC-07 | Switching methodology changes the preview and the keeper's eligible set (e.g. an asset passing DJIM but failing AAOIFI) | FR-08, 51 |
| AC-08 | `purification-check.ts` prints $2.49 ± $0.01 for AAPLx $1,000 over 6 Oct 2025 → 6 Oct 2026, and [N] with its rate source | FR-60 |
| AC-09 | A CRE run with `--limits default` passes; grep finds no `Date.now`/`new Date` in the workflow | FR-25, 27 |
| AC-10 | README, deck and video contain no "certified"/"guaranteed"; all 3 proof links and the video open logged-out | NFR-R2, R3 |
| AC-11 | Browser network tab shows no API keys; `/api/rpc` rejects a non-allow-listed method | FR-56, NFR-S7 |
| AC-12 | Owner can withdraw while `paused = true` | FR-05 |

---

## 6. Out of scope (explicit)

- Shariah certification, a fatwa, or a Shariah board (roadmap only).
- Selling / auto-divesting holdings after a downgrade (roadmap; new buys fail, owner can withdraw).
- DON deployment on the critical path (only if `cre account access` is granted in time).
- Any LLM in the decision path (weights, approval, trades).
- Lending, LP, yield, perps, Pendle, Kamino integrations (they're blocked by design).
- Jupiter Ultra/RFQ routes (can't be sandwiched by our guard).
- Non-xStock tokenized equities (Ondo GM etc.), other chains' vaults.
- Fiat on-ramp, KYC, US-person gating beyond a disclaimer.
- Fee collection / AUM fee on-chain.
- Mobile app, notifications, WebSocket subscriptions.
- Audit, verified builds, multisig upgrade authority (roadmap).
- Zakat calculation.

---

## 7. Open questions (ask now)

1. Organisers: exact submission deadline, format (video length, live vs recorded demo), multi-track rules, whether `simulate --broadcast` counts for CRE, whether Solana-only counts for NOWNodes.
2. Chainlink mentors: fast-track `cre account access`.
3. Halal Terminal: `purification_rate` units; MSTR verdict; quota bump for hackathons.
4. Teammate availability (sets @B or the solo schedule).
