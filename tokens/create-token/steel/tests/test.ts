import { Buffer } from 'node:buffer';
import { ASSOCIATED_PROGRAM_ID } from '@coral-xyz/anchor/dist/cjs/utils/token';
import { PROGRAM_ID as TOKEN_METADATA_PROGRAM_ID } from '@metaplex-foundation/mpl-token-metadata';
import { TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync } from '@solana/spl-token';
import {
  Connection,
  Keypair,
  PublicKey,
  SYSVAR_RENT_PUBKEY,
  SystemProgram,
  Transaction,
  TransactionInstruction,
  sendAndConfirmTransaction,
} from '@solana/web3.js';
import * as borsh from 'borsh';

function createKeypairFromFile(path: string): Keypair {
  return Keypair.fromSecretKey(Buffer.from(JSON.parse(require('node:fs').readFileSync(path, 'utf-8'))));
}

class Assignable {
  constructor(properties) {
    for (const [key, value] of Object.entries(properties)) {
      this[key] = value;
    }
  }
}

class CreateTokenArgs extends Assignable {
  toBuffer() {
    const buffer = new ArrayBuffer(320);
    const view = new DataView(buffer);
    view.setBigUint64(8, 10000n);
    return buffer;
  }
}
const CreateTokenArgsSchema = new Map([
  [
    CreateTokenArgs,
    {
      kind: 'struct',
      fields: [
        ['token_title', 'string'],
        ['token_symbol', 'string'],
        ['token_uri', 'string'],
        ['token_decimals', 'u8'],
      ],
    },
  ],
]);

describe('Create Tokens!', async () => {
  // const connection = new Connection(`http://localhost:8899`, 'confirmed');
  const connection = new Connection('http://localhost:8899', 'confirmed');
  const payer = createKeypairFromFile(`${require('node:os').homedir()}/.config/solana/id.json`);
  const program = createKeypairFromFile('./target/deploy/create_token_program-keypair.json');

  it('Create an SPL Token!', async () => {
    const mintKeypair: Keypair = Keypair.generate();

    const metadataAddress = PublicKey.findProgramAddressSync(
      [Buffer.from('metadata'), TOKEN_METADATA_PROGRAM_ID.toBuffer(), mintKeypair.publicKey.toBuffer()],
      TOKEN_METADATA_PROGRAM_ID,
    )[0];

    // SPL Token default = 9 decimals
    //
    const instructionData = new CreateTokenArgs({
      token_title: 'Solana Gold',
      token_symbol: 'GOLDSOL',
      token_uri: 'https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/spl-token.json',
      token_decimals: 9,
    });

    const ix = new TransactionInstruction({
      keys: [
        { pubkey: payer.publicKey, isSigner: true, isWritable: true }, // Mint authority account
        { pubkey: metadataAddress, isSigner: false, isWritable: true }, // Metadata account
        { pubkey: mintKeypair.publicKey, isSigner: true, isWritable: true }, // Mint account
        {
          pubkey: getAssociatedTokenAddressSync(mintKeypair.publicKey, payer.publicKey, true),
          isSigner: false,
          isWritable: true,
        }, // Token account
        {
          pubkey: TOKEN_METADATA_PROGRAM_ID,
          isSigner: false,
          isWritable: false,
        }, // Metadata program
        { pubkey: ASSOCIATED_PROGRAM_ID, isSigner: false, isWritable: false }, // Associated token program
        { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false }, // Token program
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false }, // System program
        { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false }, // Rent account
        {
          pubkey: TOKEN_METADATA_PROGRAM_ID,
          isSigner: false,
          isWritable: false,
        }, // Token metadata program
      ],
      programId: program.publicKey,
      data: instructionData.toBuffer(),
    });

    const sx = await sendAndConfirmTransaction(connection, new Transaction().add(ix), [payer, mintKeypair]);

    console.log('Success!');
    console.log(`   Mint Address: ${mintKeypair.publicKey}`);
    console.log(`   Tx Signature: ${sx}`);
  });

  it('Create an NFT!', async () => {
    const mintKeypair: Keypair = Keypair.generate();

    const metadataAddress = PublicKey.findProgramAddressSync(
      [Buffer.from('metadata'), TOKEN_METADATA_PROGRAM_ID.toBuffer(), mintKeypair.publicKey.toBuffer()],
      TOKEN_METADATA_PROGRAM_ID,
    )[0];

    // NFT default = 0 decimals
    const instructionData = new CreateTokenArgs({
      token_title: 'Homer NFT',
      token_symbol: 'HOMR',
      token_uri: 'https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/nft.json',
      token_decimals: 9,
    });

    const ix = new TransactionInstruction({
      keys: [
        { pubkey: payer.publicKey, isSigner: true, isWritable: true }, // Mint authority account
        { pubkey: metadataAddress, isSigner: false, isWritable: true }, // Metadata account
        { pubkey: mintKeypair.publicKey, isSigner: true, isWritable: true }, // Mint account
        {
          pubkey: getAssociatedTokenAddressSync(mintKeypair.publicKey, payer.publicKey, true),
          isSigner: false,
          isWritable: true,
        }, // Token account
        {
          pubkey: TOKEN_METADATA_PROGRAM_ID,
          isSigner: false,
          isWritable: false,
        }, // Metadata program
        { pubkey: ASSOCIATED_PROGRAM_ID, isSigner: false, isWritable: false }, // Associated token program
        { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false }, // Token program
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false }, // System program
        { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false }, // Rent account
        {
          pubkey: TOKEN_METADATA_PROGRAM_ID,
          isSigner: false,
          isWritable: false,
        }, // Token metadata program
      ],
      programId: program.publicKey,
      data: instructionData.toBuffer(),
    });

    const sx = await sendAndConfirmTransaction(connection, new Transaction().add(ix), [payer, mintKeypair]);

    console.log('Success!');
    console.log(`   Mint Address: ${mintKeypair.publicKey}`);
    console.log(`   Tx Signature: ${sx}`);
  });
});
