import { 
    Logger 
} from "log4js";
import { 
    Token
} from "@uniswap/sdk-core";

import {
    Address
} from "../../commons";
import {
    Network
} from "../../uniswap";

export interface Trader {
    net: Network;
    target: Token;
    base: Token;
    poolAddr: Address;
    config: Config;
    metrics: Partial<Metrics>;
    log?: Logger;
};

export interface Config {
    buyDiff: number;
    sellDiff: number;
    tries: number;
    confirmations: number;
    timeout: number;
    speedUp: boolean;
    gasLimit: number;
    slippageTolerance: number;
    recover: boolean;
    path: string;
};

export interface Metrics {
    meta: MetaMetrics;
    buy: SwapMetrics;
    sell: SwapMetrics & SummaryMetrics;
};

export interface MetaMetrics {
    chainId: number;
    trader: Address;
    pool: Address;
    target: string;
    base: string;
    fail?: boolean;
};

export interface SwapMetrics {
    tx: string;
    type: TradeType;
    tokenIn: string;
    tokenOut: string;
    price$: number;
    amountIn: number;
    amountOut: number;
    amountIn$: number;
    amountOut$: number;
    amountDiff$: number;
    gasUsed: number;
    gasUsed$: number;
    errors: number;
    start: Date;
    end: Date;
    time: string;
    fail?: boolean
};

export interface SummaryMetrics {
    price0$: number;
    price1$: number;
    diff: number;
    totalGasUsed$: number;
    totalAmountDiff$: number;
    gain$: number;
    net$: number;
    totalTime: string;
};

export enum Protocol {
    V2 = "V2",
    V3 = "V3"
};

export enum TradeType {
    BUY = "BUY",
    SELL = "SELL"
};
