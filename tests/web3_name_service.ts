import * as anchor from "@coral-xyz/anchor";
import { Program, BN  } from "@coral-xyz/anchor";
import { sha256 } from 'js-sha256';
import { PublicKey, Keypair } from "@solana/web3.js";
import { Web3NameService } from "../target/types/web3_name_service";
import { Buffer } from "buffer";

import testClassKeyPair from "/home/f/wallet/test2.json";
import testPayerKeypair from "/home/f/wallet/test1.json";
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";


describe("web3_name_services", () => {
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

  const ownerStr = payer.publicKey.toBase58();
  const {nameAccountKey: nameRecordAccount} = getSeedAndKey(
    program.programId, getHashedName(ownerStr), null
  )

  const ipfsHash = "QmPu4ZT2zPfyVY8CA2YBzqo9HfAV79nDuuf177tMrQK1py";
  console.log("ipfs length:", ipfsHash.length);
  const ipfsBytes = Buffer.from(ipfsHash, 'utf-8');

  const hashed_name = getHashedName(root_name);
  console.log("hashed_name:", hashed_name.length);
  const hashed_owner = getHashedName(payer.publicKey.toBase58());
  console.log("hashed_owner:", hashed_owner.length);

  const lamports = new BN(10000000);
  const space = 0; 

  const owner = payer.publicKey;
  const baseData = {
        name: root_name,
        owner: owner,
        lamports: lamports,
        space: space,
        ipfs: ipfsBytes,
    };
  
  it("this is create root domain test", async () => {
    //GLy1fKq1R2CmCCGBdXMARo9X7Y4dH8fVPECVXKP5hN5Y
    console.log("calculate:", nameAccountKey.toBase58());
    //39ZheXmgAQQ19RqyA7xrwZEFPhw3xYqRe4geDWPNziDi
    console.log("record:", nameRecordAccount.toBase58())
    //AoA4LgBAYszA1Ku6QbVwPkUzu1bu324SonNcJuGg1DNY
    const tx = await program.methods
      .create(baseData)
      .accounts({
        nameAccount: nameAccountKey,
        recordAccount: nameRecordAccount,
        payer: payer.publicKey,
        rootDomainOpt: null,
      })
      .signers([payer])
      .rpc();

  });

});



export const WEB3_NAME_SERVICE_ID = new PublicKey("62u16JAgRauejvCwT728NrnNtJBYSgR4zVc5rkZCYNnd");


function getHashedName(name: string){
  const HASH_PREFIX = "WEB3 Name Service";
  const rawHash = HASH_PREFIX + name;
  const hashValue = sha256(rawHash);
  return new Uint8Array(Buffer.from(hashValue, 'hex'));
}

function getSeedAndKey(
  programid: PublicKey,
  hashedName: Uint8Array, // 32 字节
  rootOpt: null | PublicKey
) {

  const seeds = [
    hashedName, 
    (rootOpt || PublicKey.default).toBytes(), 
  ];

  const [nameAccountKey, bump] = PublicKey.findProgramAddressSync(
    seeds, // 直接传入 [Uint8Array, Uint8Array]
    programid
  );

  return { nameAccountKey, seeds: [...hashedName, ...(rootOpt?.toBytes() || PublicKey.default.toBytes()), bump] };
}

