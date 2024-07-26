import bs58 from "bs58";
var prompt = require("prompt-sync")();

function base58ToWallet() {
  const base58 = prompt("Enter your private key: ");
  const wallet = bs58.decode(base58);
  console.log(wallet);
}

function walletToBase58() {
  const arrayBytes = prompt("Enter your keypair: ");
  const base58 = bs58.encode(JSON.parse(arrayBytes));
  console.log(base58);
}
function usage() {
  console.log("Usage: node convert.js [base58|wallet]");
  process.exit(1);
}

// Use the command line to determine which conversion to perform
const args = process.argv.slice(2);
if (args.length < 1) {
  usage();
}

if (args[0] === "base58") {
  base58ToWallet();
} else if (args[0] === "wallet") {
  walletToBase58();
} else {
  usage();
}
