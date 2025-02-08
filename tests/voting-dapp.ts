import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VotingDapp} from "../target/types/voting_dapp"; // Ensure this path is correct and the file exists
import key from "../key.json";      
import {assert }  from "chai";      

describe("voting-dapp", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.VotingDapp as Program<VotingDapp>;

    // Generate keypair from the imported key
  const secretKey = Uint8Array.from(key);
  const wallet = anchor.web3.Keypair.fromSecretKey(secretKey);


    const [admin_pda,] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("admin")],
      program.programId
    )
  
    // const admin_account = await pg.program.account.adminKey.fetch(admin_pda);
    const [vote_pda,] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("candidate"), wallet.publicKey.toBuffer()],
      program.programId
    );
  
    it("initialize the admin", async () => {
  
    const tx = await program.methods
         .initializeAdmin()
         .accounts({
           admin: admin_pda,
           signer: wallet.publicKey,
           systemProgram: anchor.web3.SystemProgram.programId,
         })
         .transaction()
   
       // Send transaction
      const txHash = await anchor.web3.sendAndConfirmTransaction(program.provider.connection ,tx, [wallet]);
   
       console.log("https://explorer.solana.com/tx/${txHash}?cluster=devnet"); 
  
      // Fetch the created account
      const adminAccount = await program.account.adminKey.fetch(admin_pda);
  
      console.log("On-chain data is:", adminAccount.adminKey.toString());
      console.log("pg key", wallet.publicKey.toString());
  
      //   assert((adminAccount.adminKey).);
      assert.strictEqual(adminAccount.adminKey.toString(), wallet.publicKey.toString());
    });
  
    it("initialize the candidates", async () => {
      const candidates = ["jerry", "emma", "frank", "sam", "david"];
  
        const tx = await program.methods
                 .initializeCandidates(candidates)
                 .accounts({
                  voteCandidates: vote_pda,
                  admin: admin_pda,
                  signer: wallet.publicKey,
                  systemProgram: anchor.web3.SystemProgram.programId
                 })
                 .transaction()
  
          const txHash = await anchor.web3.sendAndConfirmTransaction(program.provider.connection, tx, [wallet]); 
          console.log("https://explorer.solana.com/tx/${txHash}?cluster=devnet"); 
  
      const votingAccount = await program.account.votingData.fetch(vote_pda);
  
      console.log("voting data account:", votingAccount.candidatesName);
      console.log("candidate array:", candidates);
      console.log("vote pda:", new anchor.web3.PublicKey(vote_pda).toBase58());
      assert.deepEqual(
        votingAccount.candidatesName, candidates
      );
  
    })
  
    it("vote candidate", async () => {
      const kp = anchor.web3.Keypair.generate();
  
      const transaction = new anchor.web3.Transaction()
  
      transaction.add(
        anchor.web3.SystemProgram.transfer({
          fromPubkey: wallet.publicKey,
          lamports: 1000000,
          toPubkey: kp.publicKey
  
        })
      )
  
      const voter_choice = "emma";
  
      const txH = await anchor.web3.sendAndConfirmTransaction(program.provider.connection, transaction, [wallet]);
      console.log(txH);
  
      const before_vote_account = await program.account.votingData.fetch(vote_pda);
      const candidate_index = before_vote_account.candidatesName.findIndex(name => name === voter_choice);
  
      const tx = await program.methods
        .voteCandidate(voter_choice)
        .accounts({
          voteCandidates: vote_pda,
          signer: kp.publicKey,
          admin: admin_pda,
          systemProgram: anchor.web3.SystemProgram.programId
        })
        .signers([kp])
        .transaction();
  
  
      const txHash = await anchor.web3.sendAndConfirmTransaction(program.provider.connection, tx, [kp]);
      console.log("https://explorer.solana.com/tx/${txHash}?cluster=devnet");
  
      const after_vote_account = await program.account.votingData.fetch(vote_pda);
  
  
      assert.deepEqual(before_vote_account.candidatesVotes[candidate_index].words[0] + 1, after_vote_account.candidatesVotes[candidate_index].words[0])
    })
  
    it("voting result", async () => {
  
      const tx = await program.methods.votingResult()
        .accounts({
          voteAccount: vote_pda,
          signer: wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        
        })
        .transaction();
      await anchor.web3.sendAndConfirmTransaction(program.provider.connection, tx, [wallet]);
  
      const vote_account = await program.account.votingData.fetch(vote_pda);
      console.log(vote_account.votingResult);
    })
  });

  

