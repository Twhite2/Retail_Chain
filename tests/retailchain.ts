// In tests/retailchain.ts

import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Retailchain } from "../target/types/retailchain";
import { assert } from "chai";

describe("retailchain", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Retailchain as Program<Retailchain>;
  
  it("Can initialize a store", async () => {
    const store = anchor.web3.Keypair.generate();
    
    await program.methods
      .initializeStore("Test Store", "Test Location")
      .accounts({
        owner: provider.wallet.publicKey,
        store: store.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([store])
      .rpc();

    const storeAccount = await program.account.store.fetch(store.publicKey);
    assert.equal(storeAccount.name, "Test Store");
    assert.equal(storeAccount.location, "Test Location");
    assert.equal(storeAccount.totalProducts, 0);
    assert.equal(storeAccount.isActive, true);
  });

  it("Can add a product to store", async () => {
    // First initialize a store
    const store = anchor.web3.Keypair.generate();
    await program.methods
      .initializeStore("Test Store", "Test Location")
      .accounts({
        owner: provider.wallet.publicKey,
        store: store.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([store])
      .rpc();

    // Then add a product
    const product = anchor.web3.Keypair.generate();
    await program.methods
      .addProduct("Test Product", "Test Description", new anchor.BN(100), new anchor.BN(10))
      .accounts({
        owner: provider.wallet.publicKey,
        store: store.publicKey,
        product: product.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([product])
      .rpc();

    const productAccount = await program.account.product.fetch(product.publicKey);
    assert.equal(productAccount.name, "Test Product");
    assert.equal(productAccount.description, "Test Description");
    assert.equal(productAccount.price.toNumber(), 100);
    assert.equal(productAccount.quantity.toNumber(), 10);
  });

  it("Can update a product", async () => {
    // First create store and product (similar to above)
    // Then update the product
    const newPrice = new anchor.BN(150);
    const newQuantity = new anchor.BN(5);

    await program.methods
      .updateProduct(newPrice, newQuantity)
      .accounts({
        owner: provider.wallet.publicKey,
        store: store.publicKey,
        product: product.publicKey,
      })
      .rpc();

    const updatedProduct = await program.account.product.fetch(product.publicKey);
    assert.equal(updatedProduct.price.toNumber(), 150);
    assert.equal(updatedProduct.quantity.toNumber(), 5);
  });
});