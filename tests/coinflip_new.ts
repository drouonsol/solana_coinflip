import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { CoinflipNew } from "../target/types/coinflip_new";

describe("coinflip_new", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.CoinflipNew as Program<CoinflipNew>;



  it("Is initialized!", async () => {

    // Test 0 

  
    //Test 1

    // Each Point Represents A Decimnal
    const flip = await program.methods.initialize(200).rpc();
    console.log("Flip Coin : ", flip);

    
    
    //Test 2
    const claim = await program.methods.claim(1).rpc()  
    console.log("Claiming :", claim);

    // Test 3 
    const deposit = await program.methods.withdraw(0).rpc();
    console.log("Flip Coin : ", deposit);
  });
});
