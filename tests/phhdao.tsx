import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { Phhdao } from '../target/types/phhdao';
import * as spl from '@solana/spl-token';
import { SystemProgram } from '@solana/web3.js';

const provider = anchor.Provider.env()
anchor.setProvider(provider);
const program = anchor.workspace.Phhdao as Program<Phhdao>;
const ownerAccount = anchor.web3.Keypair.generate();

describe('phhdao', () => {
  const amount = new anchor.BN(500);

  
  it('Deposit', async () => {
    const signature = await program.provider.connection.requestAirdrop(ownerAccount.publicKey, 1000000000);
    await program.provider.connection.confirmTransaction(signature);

    const [mint, mintBump] = await anchor.web3.PublicKey.findProgramAddress([], program.programId);
    
    let ourAssociatedTokens = await spl.Token.getAssociatedTokenAddress(
      spl.ASSOCIATED_TOKEN_PROGRAM_ID,
      spl.TOKEN_PROGRAM_ID,
      mint,
      program.provider.wallet.publicKey,
    );

    await program.rpc.handleDeposit(amount, mintBump, {
      accounts: {
        payer: ownerAccount.publicKey, //wallet.publicKey
        treasuryAccount: "ANPzsMRRUsAhCjgG7VQxAC1hYFJChofQ239H33J12Gnj",
        mint: mint,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: spl.TOKEN_PROGRAM_ID,
        associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        userTokenAcc: ourAssociatedTokens
      },
      signers: [ownerAccount],
    } as any)
  });

  // it('Gov token Mint', async () => {
  //   await program.rpc.handleDeposit(amount, {
  //     accounts: {
  //       owner: ownerAccount.publicKey,
  //       treasuryAccount: "ANPzsMRRUsAhCjgG7VQxAC1hYFJChofQ239H33J12Gnj",
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //     },
  //     signers: [ownerAccount],
  //   } as any)
  // });

  // describe('token-studies', () => {

  //   it('Is initialized!', async () => {
  
  //     const [mint, mintBump] = await anchor.web3.PublicKey.findProgramAddress([], program.programId);
  
  //     let ourAssociatedTokens = await spl.Token.getAssociatedTokenAddress(
  //       spl.ASSOCIATED_TOKEN_PROGRAM_ID,
  //       spl.TOKEN_PROGRAM_ID,
  //       mint,
  //       program.provider.wallet.publicKey,
  //     );
  
  //     await program.rpc.mintGovToken(mintBump, {
  //       accounts: {
  //         mint: mint,
  //         payer: program.provider.wallet.publicKey,
  //         destination: ourAssociatedTokens,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //         tokenProgram: spl.TOKEN_PROGRAM_ID,
  //         associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
  //         rent: anchor.web3.SYSVAR_RENT_PUBKEY
  //       },
  //     });
  
  //     let nicelyParsedMint = await fetchMint(mint);
  //     let nicelyParsedDestinationRightAfterMint = await fetchTokenAccount(ourAssociatedTokens);
  //     debugger;
  
  //     // await program.provider.connection.confirmTransaction(
  //     //   await program.rpc.airdrop(mintBump, {
  //     //     accounts: {
  //     //       mint: mint,
  //     //       destination: destination,
  //     //       payer: program.provider.wallet.publicKey,
  //     //       systemProgram: anchor.web3.SystemProgram.programId,
  //     //       tokenProgram: spl.TOKEN_PROGRAM_ID,
  //     //       associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
  //     //       rent: anchor.web3.SYSVAR_RENT_PUBKEY
  //     //     }
  //     //   }),
  //     //   "finalized"
  //     // );
  
  //     let nicelyParsedDestination = await fetchTokenAccount(ourAssociatedTokens);
  
  //     let friend = anchor.web3.Keypair.generate();
  //     let friendsAssociatedTokenAccount = await spl.Token.getAssociatedTokenAddress(
  //       spl.ASSOCIATED_TOKEN_PROGRAM_ID,
  //       spl.TOKEN_PROGRAM_ID,
  //       mint,
  //       friend.publicKey,
  //     );
  
  //     let ix = spl.Token.createAssociatedTokenAccountInstruction(
  //       spl.ASSOCIATED_TOKEN_PROGRAM_ID,
  //       spl.TOKEN_PROGRAM_ID,
  //       mint,
  //       friendsAssociatedTokenAccount,
  //       friend.publicKey,
  //       program.provider.wallet.publicKey
  //     );
  
  //     let tx = new anchor.web3.Transaction();
  //     tx.add(ix);
  //     // tx.recentBlockhash = await (await program.provider.connection.getRecentBlockhash()).blockhash;
  //     // tx = await program.provider.wallet.signTransaction(tx);
  //     await program.provider.send(tx);
  
  
  //     tx = new anchor.web3.Transaction();
  //     tx.add(
  //       spl.Token.createTransferInstruction(
  //         spl.TOKEN_PROGRAM_ID,
  //         ourAssociatedTokens,
  //         friendsAssociatedTokenAccount,
  //         program.provider.wallet.publicKey,
  //         [],
  //         1
  //       )
  //     );
  //     await program.provider.send(tx);
  
  //     let friendsTokens = await fetchTokenAccount(friendsAssociatedTokenAccount);
  //     let ourUpdatedTokens = await fetchTokenAccount(ourAssociatedTokens);
  
  //     let friendsTokensAfterBurn = await fetchTokenAccount(friendsAssociatedTokenAccount);
  //     debugger;
  
  //   });
  // });
});

async function fetchMint(address: anchor.web3.PublicKey): Promise<Object> {
  let mintAccountInfo = await program.provider.connection.getAccountInfo(address);
  return spl.MintLayout.decode(mintAccountInfo.data);
}

async function fetchTokenAccount(address: anchor.web3.PublicKey): Promise<Object> {
  let tokenAccountInfo = await program.provider.connection.getAccountInfo(address);
  return spl.AccountLayout.decode(tokenAccountInfo.data);
}

const sleep = ms => new Promise(awaken => setTimeout(awaken, ms));


/*
- Get clone scaffold from soldev app
- anchor web3js to be used
- use anchor exported types maybe instead of IDL
- see solana twitter dapp from scratch section 8 for connecting to wallet and submitting transaction
- use solana wallet adapter react
*/