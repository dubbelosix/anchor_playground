import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { Ank } from '../target/types/ank';

describe('ank', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Ank as Program<Ank>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
