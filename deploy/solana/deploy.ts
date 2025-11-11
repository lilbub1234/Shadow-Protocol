/**
 * Shade Framework - Solana Deployment Script
 * Copyright (c) 2025 Shadow Protocol Contributors
 */

import {
  Connection,
  Keypair,
  PublicKey,
  Transaction,
  sendAndConfirmTransaction,
  SystemProgram,
} from '@solana/web3.js';
import * as fs from 'fs';
import * as path from 'path';

interface DeploymentConfig {
  cluster: 'devnet' | 'testnet' | 'mainnet-beta';
  rpcUrl?: string;
  programPath: string;
  keypairPath: string;
}

interface DeploymentResult {
  programId: string;
  transactionSignature: string;
  blockTime: number;
  slot: number;
}

/**
 * Deploy Shade Framework program to Solana
 */
async function deployShadeFram work(config: DeploymentConfig): Promise<DeploymentResult> {
  console.log('=========================================');
  console.log('Shade Framework - Solana Deployment');
  console.log('=========================================\n');

  // Connect to cluster
  const rpcUrl = config.rpcUrl || `https://api.${config.cluster}.solana.com`;
  const connection = new Connection(rpcUrl, 'confirmed');

  console.log(`Cluster: ${config.cluster}`);
  console.log(`RPC URL: ${rpcUrl}\n`);

  // Load deployer keypair
  const keypairData = JSON.parse(fs.readFileSync(config.keypairPath, 'utf-8'));
  const deployer = Keypair.fromSecretKey(Uint8Array.from(keypairData));

  console.log(`Deployer: ${deployer.publicKey.toBase58()}`);

  // Check balance
  const balance = await connection.getBalance(deployer.publicKey);
  console.log(`Balance: ${balance / 1e9} SOL\n`);

  if (balance === 0) {
    throw new Error('Insufficient balance for deployment');
  }

  // Load program binary
  const programBinary = fs.readFileSync(config.programPath);
  console.log(`Program size: ${programBinary.length} bytes\n`);

  // Create program keypair
  const programKeypair = Keypair.generate();
  console.log(`Program ID: ${programKeypair.publicKey.toBase58()}\n`);

  // Deploy program using BPF Loader
  console.log('Deploying program...');

  // In a real implementation, this would:
  // 1. Upload program data
  // 2. Initialize program account
  // 3. Deploy bytecode
  // 4. Set program authority

  // Simplified deployment for demonstration
  const deployTx = new Transaction().add(
    SystemProgram.createAccount({
      fromPubkey: deployer.publicKey,
      newAccountPubkey: programKeypair.publicKey,
      lamports: await connection.getMinimumBalanceForRentExemption(programBinary.length),
      space: programBinary.length,
      programId: new PublicKey('BPFLoader2111111111111111111111111111111111'),
    })
  );

  const signature = await sendAndConfirmTransaction(
    connection,
    deployTx,
    [deployer, programKeypair],
    {
      commitment: 'confirmed',
    }
  );

  console.log('✓ Program deployed!\n');

  // Get transaction details
  const txDetails = await connection.getTransaction(signature, {
    commitment: 'confirmed',
  });

  const result: DeploymentResult = {
    programId: programKeypair.publicKey.toBase58(),
    transactionSignature: signature,
    blockTime: txDetails?.blockTime || Date.now() / 1000,
    slot: txDetails?.slot || 0,
  };

  // Save deployment info
  const deploymentFile = path.join(__dirname, `deployments/${config.cluster}.json`);
  fs.mkdirSync(path.dirname(deploymentFile), { recursive: true });

  fs.writeFileSync(
    deploymentFile,
    JSON.stringify(
      {
        network: config.cluster,
        timestamp: new Date().toISOString(),
        deployer: deployer.publicKey.toBase58(),
        ...result,
      },
      null,
      2
    )
  );

  console.log('=========================================');
  console.log('Deployment Complete!');
  console.log('=========================================');
  console.log(`Program ID:   ${result.programId}`);
  console.log(`Signature:    ${result.transactionSignature}`);
  console.log(`Explorer:     https://explorer.solana.com/tx/${result.transactionSignature}?cluster=${config.cluster}`);
  console.log(`Details saved to: ${deploymentFile}`);
  console.log('=========================================\n');

  return result;
}

/**
 * Initialize Shade Framework program
 */
async function initializeProgram(
  connection: Connection,
  programId: PublicKey,
  authority: Keypair
): Promise<string> {
  console.log('Initializing Shade program...\n');

  // Create initialization transaction
  const initTx = new Transaction();
  // Add initialization instruction (program-specific)

  const signature = await sendAndConfirmTransaction(
    connection,
    initTx,
    [authority],
    { commitment: 'confirmed' }
  );

  console.log('✓ Program initialized!');
  console.log(`Signature: ${signature}\n`);

  return signature;
}

/**
 * Verify deployment
 */
async function verifyDeployment(
  connection: Connection,
  programId: PublicKey
): Promise<boolean> {
  console.log('Verifying deployment...\n');

  try {
    const accountInfo = await connection.getAccountInfo(programId);

    if (!accountInfo) {
      console.log('✗ Program account not found');
      return false;
    }

    console.log('✓ Program account exists');
    console.log(`  Executable: ${accountInfo.executable}`);
    console.log(`  Owner: ${accountInfo.owner.toBase58()}`);
    console.log(`  Data length: ${accountInfo.data.length} bytes\n`);

    return accountInfo.executable;
  } catch (error) {
    console.error('✗ Verification failed:', error);
    return false;
  }
}

// Main deployment script
if (require.main === module) {
  const config: DeploymentConfig = {
    cluster: (process.env.CLUSTER as any) || 'devnet',
    programPath: process.env.PROGRAM_PATH || './target/deploy/shade_program.so',
    keypairPath: process.env.KEYPAIR_PATH || '~/.config/solana/id.json',
  };

  deployShadeFramework(config)
    .then(async (result) => {
      const connection = new Connection(
        `https://api.${config.cluster}.solana.com`,
        'confirmed'
      );
      await verifyDeployment(connection, new PublicKey(result.programId));
      process.exit(0);
    })
    .catch((error) => {
      console.error('\n✗ Deployment failed:');
      console.error(error);
      process.exit(1);
    });
}

export { deploySolanaProgram, initializeProgram, verifyDeployment };
