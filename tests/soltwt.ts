import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Soltwt } from "../target/types/soltwt";

describe("soltwt", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Soltwt as Program<Soltwt>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
