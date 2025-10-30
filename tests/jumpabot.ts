import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Jumpabot } from "../target/types/jumpabot";
import { BN } from "bn.js";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from '@solana/web3.js';

describe("jumpabot", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.jumpabot as Program<Jumpabot>;

  // helpers
  const getGroupPDA = (groupName: String, signerPubkey: PublicKey) => {
    return anchor.web3.PublicKey.findProgramAddressSync([
      Buffer.from("GROUP_SEED", "utf-8"),
      Buffer.from(groupName.toUpperCase()),
      signerPubkey.toBytes()
    ], program.programId);
  }

  const getMemberPDA = (groupPDA: PublicKey, signerPubkey: PublicKey) => {
    return  anchor.web3.PublicKey.findProgramAddressSync([
      Buffer.from("MEMBER_SEED"),
      groupPDA.toBytes(),
      signerPubkey.toBytes()
    ], program.programId);
  }

  const joinGroup = async (owner: PublicKey, memberName: string, signer: Keypair, groupPDA: PublicKey, memberProfile: PublicKey) => {
    let txsig = await program.methods.joinGroup(memberName)
    .accounts({
      owner: owner,
      signer: signer.publicKey,
      group: groupPDA,
      member_profile: member1PDA
    })
    .signers([signer])
    .rpc();
    await provider.connection.confirmTransaction(txsig);
    return txsig;
  }

  const addTrader = async (signer: Keypair, groupPDA: PublicKey, trader: PublicKey) => {
    let txsig = await program.methods.addTrader()
    .accounts({
      signer: signer.publicKey,
      group: groupPDA,
      trader: trader
    })
    .signers([signer])
    .rpc();
    await provider.connection.confirmTransaction(txsig);
  }

  const removeTrader = async (signer: Keypair, groupPDA: PublicKey, trader: PublicKey) => {
    let txsig = await program.methods.removeTrader()
    .accounts({
      signer: signer.publicKey,
      group: groupPDA,
      trader: trader
    })
    .signers([signer])
    .rpc();
    await provider.connection.confirmTransaction(txsig);
  }

  const airdrop = async (pubkey: PublicKey, amount: number) => {
    const tx = await provider.connection.requestAirdrop(pubkey, amount * anchor.web3.LAMPORTS_PER_SOL);
    await provider.connection.confirmTransaction(tx);
  }

  const groupName = "Blaze Traders";
  const entry_capital = 5 * anchor.web3.LAMPORTS_PER_SOL;
  const vote_threshold = 60;
  
  const adminKeypair = anchor.web3.Keypair.generate();
  const member1kp = anchor.web3.Keypair.generate();
  const member2kp = anchor.web3.Keypair.generate();
  const member3kp = anchor.web3.Keypair.generate();
  const member4kp = anchor.web3.Keypair.generate();
  const member5kp = anchor.web3.Keypair.generate();

  const [groupPDA] = getGroupPDA(groupName, adminKeypair.publicKey);

  const adminName = "Meme Lord";
  const member1Name = "Membe 1";
  const member2Name = "Membe 2";
  const member3Name = "Membe 3";
  const member4Name = "Membe 4";
  const member5Name = "Membe 5";


  const [adminMemberPDA] = getMemberPDA(groupPDA, adminKeypair.publicKey);
  const [member1PDA] = getMemberPDA(groupPDA, member1kp.publicKey);
  const [member2PDA] = getMemberPDA(groupPDA, member2kp.publicKey);
  const [member3PDA] = getMemberPDA(groupPDA, member3kp.publicKey);
  const [member4PDA] = getMemberPDA(groupPDA, member4kp.publicKey);
  const [member5PDA] = getMemberPDA(groupPDA, member5kp.publicKey);



  before("airdrop", async () => {
    await airdrop(adminKeypair.publicKey, 5.5);
    await airdrop(member1kp.publicKey, 5.5);
    await airdrop(member2kp.publicKey, 5.5);
    await airdrop(member3kp.publicKey, 5.5);
    await airdrop(member4kp.publicKey, 5.5);
    await airdrop(member5kp.publicKey, 5.5);
  });

  it("create group", async () => {
    let txsig = await program.methods.initializeGroup(groupName, adminName, new BN(entry_capital), vote_threshold)
    .accounts({
      signer: adminKeypair.publicKey,
      group: groupPDA,
      memberProfile: adminMemberPDA
    })
    .signers([adminKeypair])
    .rpc();

    await provider.connection.confirmTransaction(txsig);

    // const group = await program.account.group.fetch(groupPDA);
    // const adminAccount = await program.account.memberProfile.fetch(adminMemberPDA);
  });

  it("joins group", async () => {
    await joinGroup(adminKeypair.publicKey, member1Name, member1kp, groupPDA, member1PDA);
    await joinGroup(adminKeypair.publicKey, member2Name, member2kp, groupPDA, member2PDA);
    await joinGroup(adminKeypair.publicKey, member3Name, member3kp, groupPDA, member3PDA);
    await joinGroup(adminKeypair.publicKey, member4Name, member4kp, groupPDA, member4PDA);
    await joinGroup(adminKeypair.publicKey, member5Name, member5kp, groupPDA, member5PDA);


    // const groupAccount = await program.account.group.fetch(groupPDA);
    // const memberAccounts = await program.account.memberProfile.all();

    // const totalCapital = await provider.connection.getBalance(groupPDA);

    // console.log("Total Capital: ", Number(totalCapital / LAMPORTS_PER_SOL), "SOL");
    // console.log("Group account details: ", groupAccount);
    // console.log("New members account details:", memberAccounts);

  });

  it("add trader", async () => {
    await addTrader(adminKeypair, groupPDA, member1kp.publicKey);
    await addTrader(adminKeypair, groupPDA, member2kp.publicKey);
    await addTrader(adminKeypair, groupPDA, member3kp.publicKey);


    const group = await program.account.group.fetch(groupPDA);

    console.log("Group account details: ", group);
  });

  it("remove trader", async () => {
    await removeTrader(adminKeypair, groupPDA, member1kp.publicKey);

    const group = await program.account.group.fetch(groupPDA);

    console.log("Group account details: ", group);
  });


  

});
