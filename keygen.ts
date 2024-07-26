import { Keypair } from "@solana/web3.js";

import fs from "fs";
import { exit } from "process";

const walletPath = "dev-wallet.json";
// Check if the wallet file already exists
if (fs.existsSync(walletPath)) {
  console.error(
    `${walletPath} already exists. Delete it to generate a new wallet.`
  );
  exit(1);
}

//Generate a new keypair
let kp = Keypair.generate();

console.log(
  `You've generated a new Solana wallet: ${kp.publicKey.toBase58()} [${
    kp.secretKey
  }]`
);

// Store keypair in `dev-wallet.json`
fs.writeFileSync(walletPath, `[${kp.secretKey}]`);
