import * as anchor from "@project-serum/anchor";
import { Keypair, PublicKey } from "@solana/web3.js";
import { Program, Spl } from "@project-serum/anchor";
import { Stream } from "../target/types/stream";

describe("stream", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Stream as Program<Stream>;
	let mint: PublicKey;
	let streamAuthority: PublicKey;

  it("Creates a mint", async () => {
		const _mint = Keypair.generate();
		mint = _mint.publicKey;

		streamAuthority = PublicKey.findProgramAddressSync(
			[Buffer.from("idk")],
			program.programId,
		)[0];

    const tx = await program
			.methods
			.createMint()
			.accounts({
				streamAuthority,
				mint: _mint.publicKey,
			})
			.signers([_mint])
			.rpc();

		const streamAuthorityAccount = await program.account.streamAuthority.fetch(streamAuthority);

		console.log('tx success', tx, streamAuthorityAccount);
  });

	it("Mint a frozen token for the current payer", async () => {
		const token = associatedTokenAddress(mint, program.provider.publicKey);
		const tx = await program
			.methods
			.mintToSelf()
			.accounts({
				token,
				mint,
				streamAuthority,
			})
			.rpc();

		const tokenProgram = Spl.token();
		const mintAccount = await tokenProgram.account.mint.fetch(mint);
		const tokenAccount = await tokenProgram.account.token.fetch(token);

		console.log('tx success', tx, mintAccount, tokenAccount);
	});

});

export function associatedTokenAddress(
  mint: PublicKey,
  wallet: PublicKey
): PublicKey {
  return anchor.utils.publicKey.findProgramAddressSync(
    [wallet.toBuffer(), TOKEN_PROGRAM_ID.toBuffer(), mint.toBuffer()],
    ASSOCIATED_TOKEN_PROGRAM_ID
  )[0];
}

export const TOKEN_PROGRAM_ID = new PublicKey(
  "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
);
export const ASSOCIATED_TOKEN_PROGRAM_ID = new PublicKey(
  "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
);
