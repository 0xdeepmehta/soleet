import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { SolTweet } from '../target/types/sol_tweet';

describe('solTweet', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.SolTweet as Program<SolTweet>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
