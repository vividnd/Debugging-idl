import { useMemo } from 'react';
import { AnchorProvider, Program } from '@coral-xyz/anchor';
import { useAnchorWallet, useConnection } from '@solana/wallet-adapter-react';
import IDL from '../se_qure.json';
import { SeQure } from '../types/se_qure';
import { PROGRAM_ID } from '../config/constants';

export function useAnchorProgram() {
  const { connection } = useConnection();
  const wallet = useAnchorWallet();

  const program = useMemo(() => {
    if (!wallet) return null;

    const provider = new AnchorProvider(
      connection,
      wallet,
      AnchorProvider.defaultOptions()
    );

    return new Program<SeQure>(IDL, provider);
  }, [connection, wallet]);

  return program;
}

export function useProvider() {
  const { connection } = useConnection();
  const wallet = useAnchorWallet();

  const provider = useMemo(() => {
    if (!wallet) return null;

    return new AnchorProvider(
      connection,
      wallet,
      AnchorProvider.defaultOptions()
    );
  }, [connection, wallet]);

  return provider;
}
