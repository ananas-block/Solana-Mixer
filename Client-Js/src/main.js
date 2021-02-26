const http = require('http')
//const fs = require('fs')
//const express = require('express')
//const app = express()
const path = require('path')
const solana = require('@solana/web3.js')
const bn = require('bn.js')
const BufferLayout = require('buffer-layout')
const token = require('@solana/spl-token')
var SHA256 = require("crypto-js/sha256");
//import { Account, Connection, PublicKey, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
//import { Account, Connection, PublicKey, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';

const solanaRPC = 'http://localhost:8899'; //'https://api.mainnet-beta.solana.com'
//Privatekey of account
let acc = "17,34,231,31,83,147,93,173,61,164,25,0,204,82,234,91,202,187,228,110,146,97,112,131,180,164,96,220,57,207,65,107,2,99,226,251,88,66,92,33,25,216,211,185,112,203,212,238,105,144,72,121,176,253,106,168,115,158,154,188,62,255,166,81";
//PublicKey of account to send stuff to
const acc_to = 'BWE49Gpc5keWZKvpdPARzqrz5o3DnAwoJNt3zSkBgRmJ';
const mixer_program_id = 'CmrAZ3KmbM6EL4bJQW8P5dJD75bPD12uwqd1f2Hx8KXD';
var connection;
const storage_account_pkey = 'Ep7nr5131gy11LhiLMMVMdaXPNw3yVifz635VruoKhni';

// Connection check
async function getNodeConnection(url) {
  connection = new solana.Connection(url, 'recent')
  const version = await connection.getVersion()
  console.log('Connection to cluster established:', url, version)
}

async function balance(address){
  //let address = acc;
  let a = new solana.PublicKey(address);
  console.log("a = ", a);
  const balance = await connection.getBalance( new solana.PublicKey(address) );

  console.log("Account:",address,"Balance:",balance / (1*10**9));
}

async function readAcc(){
  const accountInfo = await connection.getAccountInfo( new solana.PublicKey(storage_account_pkey) )
  const data = Buffer.from(accountInfo.data)
  //console.dir(data, { depth: null });
  for(const item of data){
    console.log(item);
  }
  console.log(data);
  /*
  const accountDataLayout = BufferLayout.struct([
    BufferLayout.u32('vc1'),
    BufferLayout.u32('vc2'),
  ]);
  console.log(accoundDataLayout);
  */
}

async function send_sol(){
  // Creating Transaction Instructions
  //let nr = Buffer.from(Uint8Array.of(0, ...new bn.BN(100000000).toArray("le", 8)));

  const privateKeyDecoded = acc.split(',').map(s => parseInt(s));
  //console.log(privateKeyDecoded);
  //console.log("------------------")

  const account = new solana.Account(privateKeyDecoded);
  //console.log(account.publicKey);
  let params = {fromPubkey: account.publicKey, toPubkey: acc_to};

  //let instructs = await solana.SystemProgram.transfer(params);
  //instructs.data = nr;
  //console.log(nr);
  //console.log(instructs);
  //Creating Transaction
  console.log("from Account : " , account.publicKey.toBase58());
  balance(account.publicKey.toBase58());

  const tx = new solana.Transaction().add(
        await solana.SystemProgram.transfer({
          fromPubkey: account.publicKey,
          toPubkey: acc_to,
          lamports: 100000000,
        }),
      );

  tx.recentBlockhash = await connection.getRecentBlockhash();
  //console.log(tx);

  let x = await solana.sendAndConfirmTransaction(
      connection,
      tx,
      [account],
      {
        commitment: 'singleGossip',
        preflightCommitment: 'singleGossip',
      },
    );
    console.log("x ", x);
}

async function send_to_sol_vault(nr){
  // Creating Transaction Instructions
  //let nr = Buffer.from(Uint8Array.of(0, ...new bn.BN(100000000).toArray("le", 8)));

  const privateKeyDecoded = acc.split(',').map(s => parseInt(s));
  //console.log(privateKeyDecoded);
  //console.log("------------------")

  const account = new solana.Account(privateKeyDecoded);
  var storage_acc = new solana.PublicKey(storage_account_pkey);

  //console.log(Buffer.from(storage_account_pkey));
  //console.log(storage_acc);
  //storage_acc.publicKey = Buffer.from(storage_account_pkey);
  //console.log(storage_acc);

  let params = {fromPubkey: account.publicKey, toPubkey: storage_account_pkey};

  //let instructs = await solana.SystemProgram.transfer(params);
  //instructs.data = nr;
  //console.log(nr);
  //console.log(instructs);
  //Creating Transaction
  console.log("Send from Account : " , account.publicKey.toBase58());
  //balance(account.publicKey.toBase58());

  // create sol transaction instruction
  const tx = new solana.Transaction().add(
        await solana.SystemProgram.transfer({
          fromPubkey: account.publicKey,
          toPubkey: storage_account_pkey,
          lamports: nr,
        }),
      );

  tx.recentBlockhash = await connection.getRecentBlockhash();

  // create new vault transaction
  let secret = SHA256("asd").toString();
  //console.log(secret);
  //first 16 byte are amount + 32 byte secret
  //var buffer = new ArrayBuffer(16 + 32);

  //var xy = new bn.BN(secret);

  //console.log("first bn", xy);
  //amount to bytearray
  var farr = new BigUint64Array(1);
  farr[0] = BigInt(nr);
  var barr = Buffer.from(farr.buffer);
  // hash to bytearray
  let buf = Buffer.from(secret, 'hex');
  //console.log("Barr Length = ",barr.length);

  // deposit instruction bytearray
  var inst_arr = new Uint8Array(1);
  inst_arr[0] = 1;
  var inst_barr = Buffer.from(inst_arr.buffer);

  //console.log(inst_barr);
  //let buf_nr = Buffer.from(nr.toString());
  const totalLength = buf.length + barr.length + 1;
  //console.log("Total Length = ",totalLength);
  const buf_t = Buffer.concat([buf,barr, inst_barr], totalLength);



  //let buffer = Buffer.from(Uint8Array.of(0, ...xy.toArray("le", 8)));

  let inst = [0.1, 2];
  const vault_tx = new solana.TransactionInstruction({
    programId: mixer_program_id,
    keys: [
      { pubkey: account.publicKey, isSigner: true, isWritable: false},
      { pubkey: storage_account_pkey, isSigner: false, isWritable: true},
    ],
    data: buf_t

  })

  tx.add(vault_tx);
  //console.log(tx);

  let x = await solana.sendAndConfirmTransaction(
      connection,
      tx,
      [account],
      {
        commitment: 'singleGossip',
        preflightCommitment: 'singleGossip',
      },
    );
    console.log("Deposit tx id ", x);
}

function compute_space(nr_users){
  var program_id_store = 32;
  var secret_store = 32;
  var amount = 4;
  var current_index = 8;
  return program_id_store + 1 +current_index + nr_users * (secret_store + amount);
}

// create new account to store vault struct
async function create_storage_acc(from_acc){
  //var lamports = 1 * 10 ** 10;
  const storage_account = new solana.Account();
  const program_acc = new solana.PublicKey(mixer_program_id);
  //console.log(program_acc.toBase58());
  const storage_space = compute_space(10);
  var acc_params = {fromPubkey: from_acc.publicKey,
    newAccountPubkey: storage_account.publicKey,
    lamports: await connection.getMinimumBalanceForRentExemption(storage_space),
    space: storage_space,
    programId: program_acc,
  };
  //console.log(acc_params);
  //const tx = new solana.TransactionInstruction();
  const tx = new solana.Transaction().add(solana.SystemProgram.createAccount(acc_params));
  tx.recentBlockhash = await connection.getRecentBlockhash();
  //tx.feePayer = from_acc.publicKey;
  console.log(tx);

  let x = await solana.sendAndConfirmTransaction(
        connection,
        tx,
        [from_acc,storage_account],
        {
          commitment: 'singleGossip',
          preflightCommitment: 'singleGossip',
        },
      );
      //console.log("Storage account creation tx ", x);
      //console.log("storage account pubkey = ", storage_account.publicKey.toBase58());
  //return storage_account.publicKey;
}

async function create_program_acc(from_acc){
  //var lamports = 1 * 10 ** 10;

  const program_acc = new solana.PublicKey(mixer_program_id);
  let seed = "vaultx1" ;
  var inst_arr = new Uint8Array(Buffer.from(seed));
  //console.dir(Buffer.from(seed));
  //console.dir(inst_arr);
  //inst_arr = Buffer.from(seed);
  //console.log(Buffer.from(seed));
  //console.log(from_acc.publicKey);

  const storage_account = await solana.PublicKey.createWithSeed(
    from_acc.publicKey,
    seed,
    program_acc
  );
  //EtD9SCEAp6ozZs8hqTcfRpMPVT8WEY7hA9iNAWdbdRzR
  //const storage_account = solana.PublicKey.createProgramAddress("vaultx1" ,program_acc);
  //console.log("----------------------------------");
  //console.log(storage_account.toBase58());
  //console.log("----------------------------------");
  //console.log(program_acc.toBase58());

  const storage_space = 569; // compute_space(10);
  var acc_params = {
    fromPubkey: from_acc.publicKey,
    newAccountPubkey: storage_account,
    basePubkey: from_acc.publicKey,
    seed: seed,
    lamports: await connection.getMinimumBalanceForRentExemption(storage_space),
    space: storage_space,
    programId: program_acc,
  };
  //console.log(acc_params);
  //const tx = new solana.TransactionInstruction();
  const tx = new solana.Transaction().add(solana.SystemProgram.createAccountWithSeed(acc_params));
  tx.recentBlockhash = await connection.getRecentBlockhash();
  //tx.feePayer = from_acc.publicKey;
  //console.log(tx);
  //console.log("-------------------------------------------")

  let x = await solana.sendAndConfirmTransaction(
        connection,
        tx,
        [from_acc],
        {
          commitment: 'singleGossip',
          preflightCommitment: 'singleGossip',
        },
      );
      console.log("Storage account creation tx ", x);
      console.log("storage account pubkey = ", storage_account.toBase58());
  //return storage_account.publicKey;

}

async function withdraw_sol_from_vault(nr){
  // Creating Transaction Instructions

  const privateKeyDecoded = acc.split(',').map(s => parseInt(s));

  const account = new solana.Account(privateKeyDecoded);
  var storage_acc = new solana.PublicKey(storage_account_pkey);

  let params = {fromPubkey: account.publicKey, toPubkey: storage_account_pkey};

  //Creating Transaction
  console.log("Tx from Account : " , account.publicKey.toBase58());

  // create sol transaction instruction
  const tx = new solana.Transaction();

  tx.recentBlockhash = await connection.getRecentBlockhash();

  // create new vault transaction
  let secret = SHA256("asd").toString();
  //console.log(secret);
  //first 16 byte are amount + 32 byte secret
  //var buffer = new ArrayBuffer(16 + 32);

  //amount to bytearray
  var farr = new BigUint64Array(1);
  farr[0] = BigInt(nr);
  var barr = Buffer.from(farr.buffer);
  // hash to bytearray
  let buf = Buffer.from(secret,'hex');
  //console.log(buf.length);
  // withdraw instruction bytearray
  var inst_arr = new Uint8Array(1);
  inst_arr[0] = 0;
  var inst_barr = Buffer.from(inst_arr.buffer);


  const totalLength = buf.length + barr.length + 1;
  const buf_t = Buffer.concat([buf,barr, inst_barr], totalLength);

  const vault_tx = new solana.TransactionInstruction({
    programId: mixer_program_id,
    keys: [
      { pubkey: account.publicKey, isSigner: true, isWritable: true},
      { pubkey: storage_account_pkey, isSigner: false, isWritable: true},
      //{ pubkey: new solana.PublicKey('11111111111111111111111111111111')},
    ],
    data: buf_t

  })

  tx.add(vault_tx);
  //console.log(tx);

  let x = await solana.sendAndConfirmTransaction(
      connection,
      tx,
      [account],
      {
        commitment: 'singleGossip',
        preflightCommitment: 'singleGossip',
      },
    );
    console.log("Withdraw tx id: ", x);
}


getNodeConnection(solanaRPC).then(async function() {

  console.log(" starting transaction");
  const privateKeyDecoded = acc.split(',').map(s => parseInt(s));

  const account = new solana.Account(privateKeyDecoded);
  //console.log(account.publicKey);
  //balance(acc);
  //balance(acc_to);
  //await send_sol();
  //var storage_acc = new solana.Account();
  //await create_program_acc(account);
  //console.log(compute_space(10));
  await send_to_sol_vault(1 * (10 ** 9));
  //readAcc();
  await withdraw_sol_from_vault(1 * (10 ** 9));

  /*
  //And back to hash
  let arr = Uint8Array.of(104, 135, 135, 216, 255, 20, 76, 80, 44, 127, 92, 255, 170, 254, 44, 197, 136, 216, 96, 121, 249, 222, 136, 48, 76, 38, 176, 203, 153, 206, 145, 198);
  let buf = Buffer.from(arr);
  console.log(buf.toString('hex'));
  */
  //balance(acc_to);

  //balance(acc);
  //balance(acc_to);
  //readAcc();



})
