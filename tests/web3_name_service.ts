import * as anchor from "@coral-xyz/anchor";
import { Program, BN  } from "@coral-xyz/anchor";
import { sha256 } from 'js-sha256';
import { PublicKey, Keypair } from "@solana/web3.js";
import { Web3NameService } from "../target/types/web3_name_service";
import { Buffer } from "buffer";

import testClassKeyPair from "/home/f/wallet/captain-solana-wallet.json";
import testPayerKeypair from "/home/f/wallet/left-solana-wallet.json";


describe("web3_name_service", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Web3NameService as Program<Web3NameService>;

  //fristly, create relative account

  //class account
    const secretKey = Uint8Array.from(testClassKeyPair)
    const domainClass = Keypair.fromSecretKey(secretKey);

  //payer
  const payerSecretKey = Uint8Array.from(testPayerKeypair)
  const payer = Keypair.fromSecretKey(payerSecretKey);;

  //get name account key
  const root_name = "aaa"

  const {nameAccountKey} = getSeedAndKey(
    program.programId, getHashedName(root_name), domainClass.publicKey, null
  )

  const hashedNameUint8 = getHashedName(root_name)
  const hashedName = Buffer.from(hashedNameUint8);
  const lamports = new BN(100000000); 
  const space = 500; 
  const owner = payer.publicKey;
  const baseData = {
        lamports: lamports,
        hashedName: hashedName,
        space: space,
        owner: owner,
        ipfs: null,
    };
  
  it("this is create root domain test", async () => {
    //GLy1fKq1R2CmCCGBdXMARo9X7Y4dH8fVPECVXKP5hN5Y
    console.log("calculate:", nameAccountKey)
    console.log("calculate:", domainClass.publicKey.toBase58())
    const tx = await program.methods
      .create(baseData)
      .accounts({
        nameAccount: nameAccountKey,
        payer: payer.publicKey,
        domainClass: domainClass.publicKey, 
        rootDomainOpt: PublicKey.default,
      })
      .signers([payer, domainClass])
      .rpc();
      console.log("create over")
  });
});



export const WEB3_NAME_SERVICE_ID = new PublicKey("BWK7ZQWjQ9fweneHfsYmof7znPr5GyedCWs2J8JhHxD3");

export const WEB3_ROOT = new PublicKey("52F3LuKrH19f8JATdXn1w9F3kFQceK3n5ticQmbjVs78");

function getHashedName(name: string){
  const HASH_PREFIX = "WEB3 Name Service";
  const rawHash = HASH_PREFIX + name;
  const hashValue = sha256(rawHash);
  return new Uint8Array(Buffer.from(hashValue, 'hex'));
}

function getSeedAndKey(
  programid: PublicKey, hashedName: Uint8Array, domainClass: PublicKey, rootOpt: null | PublicKey ){
  
  let seeds = new Uint8Array([...hashedName]);
  seeds = new Uint8Array([...seeds, ...domainClass.toBytes()]);
  
  const rootDomain = rootOpt || PublicKey.default;
  seeds = new Uint8Array([...seeds, ...rootDomain.toBytes()]);

  const seedChunks = [];
  for (let i = 0; i < seeds.length; i += 32) {
      const chunk = seeds.slice(i, i + 32);
      seedChunks.push(chunk);
  }

  const [nameAccountKey, bump] = PublicKey.findProgramAddressSync(
      seedChunks,
      programid
  );

  seeds = new Uint8Array([...seeds, bump]);

  return {nameAccountKey, seeds};
}

