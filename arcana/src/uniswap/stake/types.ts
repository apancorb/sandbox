import { 
    Logger 
} from "log4js";

import {
    Address
} from "../../commons";
import {
    Network,
    Pool
} from "../../uniswap";

export interface Staker {
    net: Network;
    pool: Pool;
    position?: Position;
    log: Logger;
    config: Config;
    metrics: Partial<Metrics>;
};

export interface Position {
    tokenId: number;
    tickLower: number;
    tickUpper: number;
    liquidity: string;
    url: string;
};

export interface Config {
    confirmations: number;
    gasLimit: number;
    maxSleep: number;
    mimSleep: number;
    numOfLowerTicks: number;
    numOfUpperTicks: number;
    slippageTolerance: number;
    speedUp: boolean;
    timeout: number;
    tries: number;
};

export interface Metrics {
    meta: MetaMetrics;
    mint: MintMetrics;
    collect: RemoveMetrics;
    remove: RemoveMetrics;
    summary: SummaryMetrics;
};

export interface MetaMetrics {
    chainId: number;
    staker: string;
    address: Address;
    recipient?: Address;
    pool: Address;
    token0: Address;
    token1: Address;
    //tickLower?: number;
    //tickUpper?: number;
    //tickLower$?: number;
    //tickUpper$?: number;
};

export interface BaseMetrics {
    tx: string;
    price0$: number;
    price1$: number;
    //token0PriceRatio: number;
    //token1PriceRatio: number;
    gas: number;
    gas$: number;
    errors: number;
    start: Date;
    end: Date;
    time: string;
    fail?: boolean;
};

export interface MintMetrics extends BaseMetrics {
    //token0Balance: number;
    //token1Balance: number;
    //token0Balance$: number;
    //token1Balance$: number;
    //tokenBalance$: number;
    staked0: number;
    staked1: number;
    staked0$: number;
    staked1$: number;
    staked$: number;
    //totalToken0Balance: number;
    //totalToken1Balance: number;
    //totalToken0Balance$: number;
    //totalToken1Balance$: number;
    //totalTokenBalance$: number;
};

export interface RemoveMetrics extends BaseMetrics {
    //tick: number;
    //tickDiff: number;
    //token0PriceRatioDiff: number;
    //token1PriceRatioDiff: number;
    removed0: number;
    removed1: number;
    removed0$: number;
    removed1$: number;
    removed$: number;
};

export interface SummaryMetrics {
    //priceDiff0$: number;
    //priceDiff1$: number;
    //impermanentDiff$: number;
    collected$: number;
    impermanent$: number;
    gas$: number;
    net$: number;
    time: string;
};
