import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Storage } from "../target/types/storage";
import { expect } from "chai";
import { before } from "mocha";

// Configure the client to use the local cluster.
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const program = anchor.workspace.Storage as Program<Storage>;
const systemProgram = anchor.web3.SystemProgram.programId;
let storageDataPDA;

describe("storage", async () => {
  it("Intialize", async () => {
    try {
      [storageDataPDA] = await anchor.web3.PublicKey.findProgramAddressSync(
        [provider.publicKey.toBuffer()],
        program.programId
      );
      const tx = await program.methods
        .initialize()
        .accounts({
          storageAccount: storageDataPDA,
          systemProgram: systemProgram,
        })
        .signers([])
        .rpc();
      console.log("🚀 Intialization transaction:", tx);
    } catch (error) {
      console.error(
        "Check if pda is already in use after initializing account",
        error.logs[3]
      );
    }
  });

  it("Account data is initialized to 100", async () => {
    // fetch data for pda
    const data = await program.account.storageData.fetch(storageDataPDA);
    // convert BN to decimal
    const value = parseInt(data.balance.toString("hex"), 16);
    expect(value).equal(100, "Value is not 100");
  });

  it("Increment balance up to 1000", async () => {
    for (let i = 0; i < 9; i++) {
      let data = await program.account.storageData.fetch(storageDataPDA);
      const oldValue = parseInt(data.balance.toString("hex"), 16);
      const tx = await program.methods
        .updateBalance()
        .accounts({
          storageAccount: storageDataPDA,
        })
        .signers([])
        .rpc();

      console.log("update transaction", tx);

      data = await program.account.storageData.fetch(storageDataPDA);
      const newValue = parseInt(data.balance.toString("hex"), 16);
      expect(newValue).equal(oldValue + 100, `Value is not ${oldValue + 100}`);
    }
    // check for 1000 value
    //

    const data = await program.account.storageData.fetch(storageDataPDA);
    const newValue = parseInt(data.balance.toString("hex"), 16);
    expect(newValue).equal(1000, `Value is not 1000`);
  });

  it("Throw error when trying to update balance at 1000", async () => {
    try {
      await program.methods
        .updateBalance()
        .accounts({
          storageAccount: storageDataPDA,
        })
        .signers([])
        .rpc();
      expect.fail("Expected an error to be thrown");
    } catch (error) {
      expect(error.error.errorMessage).to.equal(
        "Balance limit of 1000 reached or exceeded."
      );
    }
  });
});
