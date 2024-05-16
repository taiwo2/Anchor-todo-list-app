import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaTodoListApp } from "../target/types/solana_todo_list_app";

describe("solana-todo-list-app", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace
    .SolanaTodoListApp as Program<SolanaTodoListApp>;
  const author = program.provider as anchor.AnchorProvider;
  it("can create a task", async () => {
    const task = anchor.web3.Keypair.generate();
    const tx = await program.methods
      .addingTask("You are awesome")
      .accounts({
        task: task.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([task])
      .rpc();
    console.log("Your transaction signature", tx);

    const taskAccount = await program.account.task.fetch(task.publicKey);
    console.log("Your task", taskAccount);

    assert.equal(
      taskAccount.author.toBase58(),
      author.wallet.publicKey.toBase58()
    );
    assert.equal(taskAccount.text, "You are awesome");
    assert.equal(taskAccount.isDone, false);
    assert.ok(taskAccount.createdAt);
    assert.ok(taskAccount.updatedAt);
  });
});
