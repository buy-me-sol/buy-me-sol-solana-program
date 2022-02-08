const anchor = require('@project-serum/anchor');
const { PublicKey } = require('@solana/web3.js');

const { SystemProgram } = anchor.web3;  

const main = async() => {
  console.log("🚀 Starting test...")

  // Create provide
  const provider = anchor.Provider.env();
  // Set provider
  anchor.setProvider(provider);
  const program = anchor.workspace.BuyMeSolSolanaProgram;

  // Create account keypair for our program to use
  const baseAccount = anchor.web3.Keypair.generate();

  // Call Initialize, pass params it needs
  let tx = await program.rpc.initialize({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAccount],
  });
  
  console.log("📝 Your transaction signature", tx);
  
  // Fetch data from account
  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 Total Creators: ', account.totalCreators.toString());
  console.log('👀 Total Supporters: ', account.totalSupporters.toString());
  
  // Call create creator account
  await program.rpc.createCreator("apraX568","Apratim Mehta",{
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    }
  });

  // Call create creator account
  await program.rpc.createSupporter("Apratim Mehta", {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    }
  });

  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 Total Creators: ', account.totalCreators.toString());
  console.log('👀 Total Supporters: ', account.totalSupporters.toString());

  // Access creator_list on the account
  console.log('👀 Creator List :', account.creatorList);
  // Access creator_list on the account
  console.log('👀 Supporter List :', account.supporterList);

  // Test Add Message
  const creatorPubkey = new PublicKey('69e2mCepiCTdZfjedvWnZfunwhYFc1qo2Tmf1CTpNoQJ')

  // Call add message
  await program.rpc.addMessage(creatorPubkey, "Nice Work 👍", "1.1",{
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    }
  });

  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  // Access messages on the account
  console.log('👀 Message :', account.messages);
}

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
};

runMain();