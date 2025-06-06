// import * as anchor from '@coral-xyz/anchor'
// import { Program } from '@coral-xyz/anchor'
// import { Keypair, PublicKey } from '@solana/web3.js'
// import { Voting } from '../target/types/voting'
// import { BankrunProvider, startAnchor } from "anchor-bankrun";
//
// const IDL = require('../target/idl/voting.json');
//
// const votingAddress = new PublicKey("DenE1esqiAydZLirKcykhFKMKVxq825x8dYc1Vix89PY");
// describe('voting', () => {
//
//   	anchor.setProvider(anchor.AnchorProvider.env());
// 	// let context;
// 	// let provider;
// 	// let votingProgram;
//
// 	beforeAll(async() => {
//
// 		// context = await startAnchor("", [{ name: "voting", programId: votingAddress }], []);
// 		// provider = new BankrunProvider(context);
// 		// votingProgram = new Program<Voting>(IDL, provider);
//
// 	})
// 	it('Initialize Poll', async () => {
// 		const context = await startAnchor("", [{ name: "voting", programId: votingAddress }], []);
// 		const provider = new BankrunProvider(context);
// 		const votingProgram = new Program<Voting>(IDL, provider);
// 		await votingProgram.methods.initializePoll(new anchor.BN(1),
// 			"What is your favourite type of Chicken?",
// 			new anchor.BN(0),
// 			new anchor.BN(1821246488)).rpc();
// 		const [pollAddress] = PublicKey.findProgramAddressSync([new anchor.BN(1).toArrayLike(Buffer, 'le', 8)],
// 			votingAddress)
// 		const poll = await votingProgram.account.poll.fetch(pollAddress);
// 		console.log("poll: ", poll);
//
// 		expect(poll.pollId.toNumber()).toEqual(1);
// 		expect(poll.description).toEqual("What is your favourite type of Chicken?");
// 		expect(poll.pollStart.toNumber()).toBeLessThan(poll.pollEnd.toNumber());
// 	});
//
// 	it("Initialize candidate", async() => {
//
// 		const pollIdBuffer = new anchor.BN(1).toArrayLike(Buffer, "le", 8)
//
// 		const [pollAddress] = PublicKey.findProgramAddressSync([Buffer.from("poll"), pollIdBuffer], votingAddress);
//
// 		await votingProgram.methods.initializeCandidate("Butter", new anchor.BN(1)).accounts({pollAccount:pollAddress}).rpc();
// 		await votingProgram.methods.initializeCandidate("Tikka", new anchor.BN(1)).accounts({pollAccount:pollAddress}).rpc();
// 		const [butterAddress] = PublicKey.findProgramAddressSync(
// 			[new anchor.BN(1).toArrayLike(Buffer, 'le', 8), Buffer.from("Butter")],
// 			votingAddress,
// 		);
//
// 		const ButterCandidate = await votingProgram.account.candidate.fetch(butterAddress);
// 		console.log(ButterCandidate);
// 	});
//
// 	it("vote", async() => {
//
//
// 	});
// });
import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Voting } from '../target/types/voting';
import { PublicKey } from '@solana/web3.js';

describe('Voting', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Voting as Program<Voting>;

  it('initializePoll', async () => {

    const [pollAddress] = PublicKey.findProgramAddressSync(
      [Buffer.from("poll"), new anchor.BN(1).toArrayLike(Buffer, "le", 8)],
      program.programId
    );

    const tx = await program.methods.initializePoll(
        new anchor.BN(1),
        new anchor.BN(0),
        new anchor.BN(1759508293),
        "test-poll",
        "description",
    )
    .rpc();

    console.log('Your transaction signature', tx);
  });

  it('initialize candidates', async () => {
    const pollIdBuffer = new anchor.BN(1).toArrayLike(Buffer, "le", 8)

    const [pollAddress] = PublicKey.findProgramAddressSync(
      [Buffer.from("poll"), pollIdBuffer],
      program.programId
    );

    const smoothTx = await program.methods.initializeCandidate(
      new anchor.BN(1), 
      "smooth",
    ).accounts({
      pollAccount: pollAddress
    })
    .rpc();

	// const smoothCandidate = awai

    const crunchyTx = await program.methods.initializeCandidate(
      new anchor.BN(1), 
      "crunchy",
    ).accounts({
      pollAccount: pollAddress
    })
    .rpc();

    console.log('Your transaction signature', smoothTx);
  });

  it('vote', async () => {

    const tx = await program.methods.vote(
      new anchor.BN(1),
      "smooth",
    )
    .rpc();

    console.log('Your transaction signature', tx);
  });
});
