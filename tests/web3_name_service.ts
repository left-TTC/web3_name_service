import * as anchor from "@coral-xyz/anchor";
import { Program, BN  } from "@coral-xyz/anchor";
import { sha256 } from 'js-sha256';
import { PublicKey, Keypair } from "@solana/web3.js";
import { Web3NameService } from "../target/types/web3_name_service";
import { Buffer } from "buffer";

import testClassKeyPair from "/home/f/wallet/test2.json";
import testPayerKeypair from "/home/f/wallet/test1.json";
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";
import { seed } from "@coral-xyz/anchor/dist/cjs/idl";


const HASH_PREFIX = "WEB3 Name Service";

export function getHashedName(name: string){
  const rawHash = HASH_PREFIX + name;
  const hashValue = sha256(rawHash);
  return new Uint8Array(Buffer.from(hashValue, 'hex'));
}

export function getSeedAndKey(
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

  return {nameAccountKey, seedChunks};
}

interface BaseData {
  name: string;
  root: PublicKey;
  hasedName: Buffer; // 注意拼写是 hasedName 不是 hashedName
  ipfs: number[] | null;
  owner: PublicKey;
}

describe("web3_name_services", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Web3NameService as Program<Web3NameService>;

  //fristly, create relative account
  //payer
  const payerSecretKey = Uint8Array.from(testPayerKeypair)
  const payer = Keypair.fromSecretKey(payerSecretKey);;

  //get name account key
  
  const root_name = "aaaa"

  const {nameAccountKey: nameAccountPDA, seedChunks: seed} = getSeedAndKey(
    program.programId, getHashedName(root_name), null)

  console.log("create:", nameAccountPDA.toBase58())

  console.log(Buffer.concat(seed).length)

  const owner = payer.publicKey;
  const baseData: BaseData = {
        name: root_name,
        root: PublicKey.default,
        hasedName: Buffer.from(getHashedName(root_name)),
        ipfs: null,
        owner: payer.publicKey,
    };
  
  it("this is create root domain test", async () => {
        try{
          const tx = await program.methods
            .create(baseData)
            .accounts({
              nameAccount: nameAccountPDA,
              rootDomainOpt: null,
              payer: payer.publicKey
            })
            .signers([payer])
            .rpc();
          console.log('Transaction successful:', tx);
        } catch (err) {
            console.error('Error creating name:', err);
        }
  });

});






