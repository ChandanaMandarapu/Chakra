export interface CrossChainIntent {
  owner: string;
  targetChainId: number;
  nonce: number;
  amount: number;
  sourceChain: string;
  targetChain: string;
  targetAddress: string;
  escrowPda: string;
  timeoutSlot: number;
  isFinalized: boolean;
  isCancelled: boolean;
}

export interface SentinelNodeStatus {
  nodeId: number;
  port: number;
  isActive: boolean;
  lastSignedIntent?: string;
}
