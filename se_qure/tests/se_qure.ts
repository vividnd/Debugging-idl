import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { SeQure } from "../target/types/se_qure";
import { randomBytes } from "crypto";
import {
  awaitComputationFinalization,
  getArciumEnv,
  getClusterAccAddress,
  getCompDefAccOffset,
  getArciumAccountBaseSeed,
  getArciumProgAddress,
  uploadCircuit,
  buildFinalizeCompDefTx,
  RescueCipher,
  deserializeLE,
  getMXEPublicKey,
  getMXEAccAddress,
  getMempoolAccAddress,
  getCompDefAccAddress,
  getExecutingPoolAccAddress,
  getComputationAccAddress,
  x25519,
} from "@arcium-hq/client";
import * as fs from "fs";
import * as os from "os";
import { expect } from "chai";

// Arcium Program ID constant
const ARCIUM_PROGRAM_ID = new PublicKey("BKck65TgoKRokMjQM3datB9oRwJ8rAj2jxPXvHXUvcL6");

describe("SeQure", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace
    .SeQure as Program<SeQure>;
  const provider = anchor.getProvider();

  type Event = anchor.IdlEvents<(typeof program)["idl"]>;
  const awaitEvent = async <E extends keyof Event>(
    eventName: E
  ): Promise<Event[E]> => {
    let listenerId: number;
    const event = await new Promise<Event[E]>((res) => {
      listenerId = program.addEventListener(eventName, (event) => {
        res(event);
      });
    });
    await program.removeEventListener(listenerId);

    return event;
  };

  // Use devnet cluster instead of local environment
  const DEVNET_CLUSTER_OFFSET = 1078779259;
  const clusterAccount = getClusterAccAddress(DEVNET_CLUSTER_OFFSET);

  it("Can create a survey", async () => {
    const owner = readKpJson(`${os.homedir()}/.config/solana/id.json`);

    // Create a survey
    const timestamp = Date.now();
    const slug = `test-survey-${timestamp}`;
    const title = "Test Survey";
    const description = "A test survey for unit testing";
    const surveyType = { basic: {} };
    const maxResponses = 100;

    console.log("Creating survey...");
    const createSig = await program.methods
      .createSurveyWithTimestamp(
        new anchor.BN(timestamp),
        slug,
        title,
        description,
        surveyType,
        maxResponses
      )
      .accounts({
        creator: owner.publicKey,
      })
      .signers([owner])
      .rpc({ skipPreflight: true, commitment: "confirmed" });

    console.log("Survey created with signature:", createSig);

    // Get the survey PDA
    const [surveyPDA] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("survey"),
        owner.publicKey.toBuffer(),
        Buffer.from(slug),
      ],
      program.programId
    );

    // Verify the survey was created
    const surveyAccount = await program.account.survey.fetch(surveyPDA);
    expect(surveyAccount.creator.toString()).to.equal(owner.publicKey.toString());
    expect(surveyAccount.title).to.equal(title);
    expect(surveyAccount.description).to.equal(description);
    expect(surveyAccount.slug).to.equal(slug);
  });

  it("Can initialize analytics computation definition", async () => {
    const owner = readKpJson(`${os.homedir()}/.config/solana/id.json`);

    console.log("Initializing analytics computation definition...");
    const initSig = await program.methods
      .initAnalyticsComputationCompDef()
      .accounts({
        payer: owner.publicKey,
        mxeAccount: getMXEAccAddress(ARCIUM_PROGRAM_ID),
        compDefAccount: getCompDefAccAddress(
          program.programId,
          Buffer.from(getCompDefAccOffset("analytics_computation")).readUInt32LE()
        ),
      })
      .signers([owner])
      .rpc({ skipPreflight: true, commitment: "confirmed" });

    console.log("Analytics computation definition initialized with signature:", initSig);
  });

  it("Can initialize quiz evaluation computation definition", async () => {
    const owner = readKpJson(`${os.homedir()}/.config/solana/id.json`);

    console.log("Initializing quiz evaluation computation definition...");
    const initSig = await program.methods
      .initQuizEvaluationCompDef()
      .accounts({
        payer: owner.publicKey,
        mxeAccount: getMXEAccAddress(ARCIUM_PROGRAM_ID),
        compDefAccount: getCompDefAccAddress(
          program.programId,
          Buffer.from(getCompDefAccOffset("quiz_evaluation")).readUInt32LE()
        ),
      })
      .signers([owner])
      .rpc({ skipPreflight: true, commitment: "confirmed" });

    console.log("Quiz evaluation computation definition initialized with signature:", initSig);
  });

});

async function getMXEPublicKeyWithRetry(
  provider: anchor.AnchorProvider,
  programId: PublicKey,
  maxRetries: number = 10,
  retryDelayMs: number = 500
): Promise<Uint8Array> {
  for (let attempt = 1; attempt <= maxRetries; attempt++) {
    try {
      const mxePublicKey = await getMXEPublicKey(provider, programId);
      if (mxePublicKey) {
        return mxePublicKey;
      }
    } catch (error) {
      console.log(`Attempt ${attempt} failed to fetch MXE public key:`, error);
    }

    if (attempt < maxRetries) {
      console.log(
        `Retrying in ${retryDelayMs}ms... (attempt ${attempt}/${maxRetries})`
      );
      await new Promise((resolve) => setTimeout(resolve, retryDelayMs));
    }
  }

  throw new Error(
    `Failed to fetch MXE public key after ${maxRetries} attempts`
  );
}

function readKpJson(path: string): anchor.web3.Keypair {
  const file = fs.readFileSync(path);
  return anchor.web3.Keypair.fromSecretKey(
    new Uint8Array(JSON.parse(file.toString()))
  );
}
