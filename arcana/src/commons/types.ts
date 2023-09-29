import {
    BigNumber,
    providers,
    Signer,
    Transaction
} from "ethers";

export type SignerOrProvider = Signer | providers.BaseProvider;

export type Address = string;

export interface TransactionResponse extends Transaction {
    hash: string;
    // Only if a transaction has been mined
    blockNumber?: number,
    blockHash?: string,
    timestamp?: number,
    confirmations: number,
    // Not optional (as it is in Transaction)
    from: string;
    // The raw transaction
    raw?: string,
    // This function waits until the transaction has been mined
    wait: (confirmations?: number) => Promise<TransactionReceipt>
};

export interface TransactionReceipt {
    to: string;
    from: string;
    contractAddress: string,
    transactionIndex: number,
    root?: string,
    gasUsed: BigNumber,
    logsBloom: string,
    blockHash: string,
    transactionHash: string,
    logs: Array<Log>,
    blockNumber: number,
    confirmations: number,
    cumulativeGasUsed: BigNumber,
    effectiveGasPrice: BigNumber,
    byzantium: boolean,
    type: number;
    status?: number
};

export interface Log {
    blockNumber: number;
    blockHash: string;
    transactionIndex: number;
    removed: boolean;
    address: string;
    data: string;
    topics: Array<string>;
    transactionHash: string;
    logIndex: number;
};
