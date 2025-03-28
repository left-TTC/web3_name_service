import * as anchor from "@coral-xyz/anchor";
import { Program, BN  } from "@coral-xyz/anchor";
import { sha256 } from 'js-sha256';
import { PublicKey, Keypair } from "@solana/web3.js";
import { Web3NameService } from "../target/types/web3_name_service";
import { Buffer } from "buffer";

import testClassKeyPair from "/home/f/wallet/test2.json";
import testPayerKeypair from "/home/f/wallet/test1.json";


describe("web3_name_service", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Web3NameService as Program<Web3NameService>;

  //fristly, create relative account


  //payer
  const payerSecretKey = Uint8Array.from(testPayerKeypair)
  const payer = Keypair.fromSecretKey(payerSecretKey);;

  //get name account key
  const root_name = "aaa"

  const {nameAccountKey} = getSeedAndKey(
    program.programId, getHashedName(root_name), null
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
    const tx = await program.methods
      .create(baseData)
      .accounts({
        nameAccount: nameAccountKey,  
        payer: payer.publicKey,
        rootDomainOpt: PublicKey.default,
      })
      .signers([payer])
      .rpc();
      console.log("create over")
  });
});



export const WEB3_NAME_SERVICE_ID = new PublicKey("BWK7ZQWjQ9fweneHfsYmof7znPr5GyedCWs2J8JhHxD3");


function getHashedName(name: string){
  const HASH_PREFIX = "WEB3 Name Service";
  const rawHash = HASH_PREFIX + name;
  const hashValue = sha256(rawHash);
  return new Uint8Array(Buffer.from(hashValue, 'hex'));
}

function getSeedAndKey(
  programid: PublicKey, hashedName: Uint8Array, rootOpt: null | PublicKey ){
  
  let seeds = new Uint8Array([...hashedName]);
  
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

