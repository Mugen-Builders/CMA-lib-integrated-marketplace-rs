#!/usr/bin/env node
import "dotenv/config";
import { ethers } from "ethers";

// ---------- CONFIG ----------
const RPC_URL = process.env.RPC_URL || "http://localhost:8545";
const PRIVATE_KEY = process.env.PRIVATE_KEY_1;
const PRIVATE_KEY_2 = process.env.PRIVATE_KEY_2;

const address1 = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266";
const inputBox = "0x59b22D57D4f067708AB0c00552767405926dc768";
const address2 = "0x70997970C51812dc3A010C7d01b50e0d17dc79C8";
const app = "0xab7528bb862fB57E8A2BCd567a2e929a0Be56a5e";
const erc20_token = "0x92C6bcA388E99d6B304f1Af3c3Cd749Ff0b591e2";
const erc20_portal = "0x9C21AEb2093C32DDbC53eEF24B873BDCd1aDa1DB";
const erc721_token = "0xc6582A9b48F211Fa8c2B5b16CB615eC39bcA653B";
const erc721_portal = "0x237F8DD094C0e47f4236f12b4Fa01d6Dae89fb87";
const amount = "150";
const tokenId = 2;

// ---------- ABIs ----------
const targetAbi = [
  "function addInput(address appContract, bytes payload)",
  "function depositERC20Tokens(address token, address appContract, uint256 value, bytes execLayerData)",
  "function depositERC721Token(address token, address appContract, uint256 tokenId, bytes baseLayerData, bytes execLayerData)"
];

const erc20Abi = [
  "function transfer(address to, uint256 value) public returns (bool)",
  "function approve(address spender, uint256 value) public returns (bool)"
];

const nftAbi = [
  "function transferFrom(address from, address to, uint256 tokenId) external",
  "function approve(address to, uint256 tokenId) external",
  "function safeMint(address to, uint256 tokenId, string memory uri) public",
  "function setApprovalForAll(address operator, bool approved) public"
];

// ---------- HELPERS ----------
const provider = new ethers.JsonRpcProvider(RPC_URL);
const signer1 = new ethers.Wallet(PRIVATE_KEY, provider);
const signer2 = new ethers.Wallet(PRIVATE_KEY_2, provider);

function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

function encodeJsonAsBytes(payloadJson) {
  const json = typeof payloadJson === "string" ? payloadJson : JSON.stringify(payloadJson);
  return ethers.hexlify(ethers.toUtf8Bytes(json));
}

// ---------- CORE ACTIONS ----------

async function addInput(appContract, hex_input, signer) {
  const contract = new ethers.Contract(inputBox, targetAbi, signer);
  console.log(`🟦 Calling addInput(appContract=${appContract})...`);
  const tx = await contract.addInput(appContract, hex_input);
  console.log("→ tx:", tx.hash);
  const receipt = await tx.wait();
  console.log("✅ addInput confirmed in block", receipt.blockNumber);
}

async function approveERC20(token, spender, amount, signer) {
  const tokenCtr = new ethers.Contract(token, erc20Abi, signer);
  const amt = ethers.parseUnits(amount.toString(), 18);
  console.log(`🟩 Approving ${spender} to spend ${amount} tokens...`);
  const tx = await tokenCtr.approve(spender, amt);
  console.log("→ tx:", tx.hash);
  const receipt = await tx.wait();
  console.log("✅ approve confirmed in block", receipt.blockNumber);
}

async function transferERC20(token, to, amount) {
  const tokenCtr = new ethers.Contract(token, erc20Abi, signer1);
  const amt = ethers.parseUnits(amount.toString(), 18);
  console.log(`🟨 Transferring ${amount} tokens to ${to}...`);
  const tx = await tokenCtr.transfer(to, amt);
  console.log("→ tx:", tx.hash);
  const receipt = await tx.wait();
  console.log("✅ transfer confirmed in block", receipt.blockNumber);
}

async function depositERC20Tokens(token, appContract, amount, execLayerData, signer) {
  const contract = new ethers.Contract(erc20_portal, targetAbi, signer);
  const bytesData =
    typeof execLayerData === "string" && execLayerData.startsWith("0x")
      ? execLayerData
      : encodeJsonAsBytes(execLayerData);
  const amt = ethers.parseUnits(amount.toString(), 18);
  console.log(`🟪 Depositing ${amount} tokens to ${appContract}...`);
  const tx = await contract.depositERC20Tokens(token, appContract, amt, bytesData);
  console.log("→ tx:", tx.hash);
  const receipt = await tx.wait();
  console.log("✅ deposit confirmed in block", receipt.blockNumber);
}

async function depositERC721Tokens(token, appContract, amount, execLayerData, baseLayerData, signer) {
  const contract = new ethers.Contract(erc721_portal, targetAbi, signer);
  const bytesData =
    typeof execLayerData === "string" && execLayerData.startsWith("0x")
      ? execLayerData
      : encodeJsonAsBytes(execLayerData);
  const bytesData2 =
    typeof baseLayerData === "string" && baseLayerData.startsWith("0x")
      ? baseLayerData
      : encodeJsonAsBytes(baseLayerData);
  const amt = ethers.parseUnits(amount.toString(), 18);
  console.log(`🟪 Depositing ${amount} tokens to ${appContract}, ...`);
  const tx = await contract.depositERC721Token(erc721_token, app, amount, bytesData, bytesData2);
  console.log("→ tx:", tx.hash);
  const receipt = await tx.wait();
  console.log("✅ deposit confirmed in block", receipt.blockNumber);
}

/** Mints an NFT */
async function mintNFT(nftAddress, to, tokenId) {
  const nft = new ethers.Contract(nftAddress, nftAbi, signer1);
  console.log(`🎨 Minting NFT tokenId=${tokenId} to ${to}...`);
  const tx = await nft.safeMint(to, tokenId, "");
  console.log("→ tx:", tx.hash);
  const receipt = await tx.wait();
  console.log("✅ NFT minted in block", receipt.blockNumber);
}

/** Approves another address to use NFT */
async function approveNFT(nftAddress, to, tokenId, signer) {
  const nft = new ethers.Contract(nftAddress, nftAbi, signer);
  console.log(`🧾 Approving ${to} to manage NFT tokenId=${tokenId}...`);
//   const tx = await nft.approve(to, tokenId);
  const tx = await nft.setApprovalForAll(erc721_portal, true);
  console.log("→ tx:", tx.hash);
  const receipt = await tx.wait();
  console.log("✅ NFT approval confirmed in block", receipt.blockNumber);
}

// ---------- MAIN EXECUTION FLOW ----------
async function main() {
  console.log("🚀 Starting contract interaction sequence...\n");

  // 2️⃣ Approve
  await approveERC20(erc20_token, erc20_portal, amount, signer1);
  await sleep(2000);

    // 3️⃣ Deposit
  await depositERC20Tokens(erc20_token, app, amount, "", signer1);
  await sleep(2000);

  await mintNFT(erc721_token, address1, tokenId);
  await sleep(2000);

  await approveNFT(erc721_token, erc721_portal, tokenId, signer1);
  await sleep(2000);

  await depositERC721Tokens(erc721_token, app, tokenId, "", "", signer1);
  await sleep(2000);

//   4️⃣ Transfer (or skip if not needed)
  await transferERC20(erc20_token, address2, amount);
  await sleep(2000);

  await approveERC20(erc20_token, erc20_portal, amount, signer2);
  await sleep(2000);

  await depositERC20Tokens(erc20_token, app, amount, "", signer2);
  await sleep(2000);

  const iface = new ethers.Interface(["function PurchaseToken(uint256)"]);
  const hex_input = iface.encodeFunctionData("PurchaseToken", [tokenId]);

  console.log("🟦 Calling::: " + hex_input);
  await addInput(app, hex_input, signer2);
  await sleep(2000);

  await addInput(app, {function_type: "purchase_token", token_id: tokenId.toString()}, signer2);
  await sleep(2000);

  console.log("\n✅ All actions completed!");
}

main().catch((err) => {
  console.error("❌ Error:", err);
  process.exit(1);
});