import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { Phhdao } from '../target/types/phhdao';
describe('phhdao', () => {
  const provider = anchor.Provider.env()
  anchor.setProvider(provider);
  const program = anchor.workspace.Phhdao as Program<Phhdao>;
  const ownerAccount = anchor.web3.Keypair.generate();
  const amount = new anchor.BN(500);

  it('Deposit', async () => {
    const signature = await program.provider.connection.requestAirdrop(ownerAccount.publicKey, 1000000000);
    await program.provider.connection.confirmTransaction(signature);

    await program.rpc.handleDeposit(amount, {
      accounts: {
        owner: ownerAccount.publicKey,
        treasuryAccount: "ANPzsMRRUsAhCjgG7VQxAC1hYFJChofQ239H33J12Gnj",
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [ownerAccount],
    } as any)
  });

  it('Gov token Mint', async () => {
    await program.rpc.handleDeposit(amount, {
      accounts: {
        owner: ownerAccount.publicKey,
        treasuryAccount: "ANPzsMRRUsAhCjgG7VQxAC1hYFJChofQ239H33J12Gnj",
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [ownerAccount],
    } as any)
  });
});