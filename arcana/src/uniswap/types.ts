import { 
    BigNumber,
    providers,
    Signer
} from "ethers";
import { 
    Token
} from "@uniswap/sdk-core";

import {
    Address
} from "../commons";

export interface Network {
    chainId: number;
    coin: Token;
    signer: Signer;
    provider: providers.BaseProvider;
    address: Address;
    recipient?: Address;
};

export interface Pool {
    address: Address;
    token0: Token;
    token1: Token;
    //immutables?: PoolImmutables;
    //mutables?: PoolMutables;
};

export interface PoolImmutables {
    factory: string;
    token0: string;
    token1: string;
    fee: number;
    tickSpacing: number;
    maxLiquidityPerTick: BigNumber;
};

export interface PoolMutables {
    liquidity: BigNumber;
    sqrtPriceX96: BigNumber;
    tick: number;
    observationIndex: number;
    observationCardinality: number;
    observationCardinalityNext: number;
    feeProtocol: number;
    unlocked: boolean;
};
