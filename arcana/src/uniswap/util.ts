import {
    BigNumber,
    Contract
} from "ethers";
import { 
    Pool,
    Position,
    nearestUsableTick
} from "@uniswap/v3-sdk";
import { 
    Token
} from "@uniswap/sdk-core";
import {
    abi as UNI_V3_POSITION_MANAGER_ABI
} from "@uniswap/v3-periphery/artifacts/contracts/NonfungiblePositionManager.sol/NonfungiblePositionManager.json";
import {
    abi as UNI_V3_SWAP_ROUTER_ABI
} from "@uniswap/v3-periphery/artifacts/contracts/SwapRouter.sol/SwapRouter.json";
import { 
    abi as UNI_V3_POOL_ABI
} from "@uniswap/v3-core/artifacts/contracts/interfaces/IUniswapV3Pool.sol/IUniswapV3Pool.json";

import {
    CHAIN_ID as MATIC_CHAIN_ID,
    getTokenPrice as getMaticTokenPrice
} from "./matic";
import {
    PoolImmutables,
    PoolMutables
} from "./types";
import {
    SignerOrProvider
} from "../commons";

export const POSITION_MANAGER_ADDRESS: string = 
    "0xC36442b4a4522E871399CD717aBDD847Ab11FE88";
export const SWAP_ROUTER_ADDRESS: string =
    "0x68b3465833fb72A70ecDF485E0e4C7bD8665Fc45";
export const POSITION_MANAGER_ABI: any =
    UNI_V3_POSITION_MANAGER_ABI;
export const SWAP_ROUTER_ABI: any =
    UNI_V3_SWAP_ROUTER_ABI;
export const POOL_ABI: any =
    UNI_V3_POOL_ABI;

export const getPositionManagerContract = (
    signerOrProvider: SignerOrProvider
): Contract => {
    return new Contract(
        POSITION_MANAGER_ADDRESS,
        POSITION_MANAGER_ABI,
        signerOrProvider
    );
};

export const getSwapRouterContract = (
    signerOrProvider: SignerOrProvider
): Contract => { 
    return new Contract(
        SWAP_ROUTER_ADDRESS,
        SWAP_ROUTER_ABI,
        signerOrProvider
    );
};

export const getPoolContract = (
    signerOrProvider: SignerOrProvider,
    poolAddress: string
): Contract => {
    return new Contract(
        poolAddress,
        POOL_ABI,
        signerOrProvider,
    );
};

export const getPool = async (
    signerOrProvider: SignerOrProvider,
    poolAddress: string,
    token0: Token,
    token1: Token
): Promise<Pool> => {
    const [immutable, mutable] = await Promise.all([
        getPoolImmutables(
            signerOrProvider,
            poolAddress
        ),
        getPoolMutables(
            signerOrProvider,
            poolAddress
        )
    ]);
    return new Pool(
        token0,
        token1,
        immutable.fee,
        mutable.sqrtPriceX96.toString(),
        mutable.liquidity.toString(),
        mutable.tick
    );
};

export const getPoolImmutables = async (
    signerOrProvider: SignerOrProvider,
    poolAddress: string
): Promise<PoolImmutables> => {
    const poolContract: Contract = getPoolContract(
        signerOrProvider,
        poolAddress
    );
    const [
        factory,
        token0,
        token1,
        fee,
        tickSpacing,
        maxLiquidityPerTick
    ] = await Promise.all([
        poolContract.factory(),
        poolContract.token0(),
        poolContract.token1(),
        poolContract.fee(),
        poolContract.tickSpacing(),
        poolContract.maxLiquidityPerTick()
    ]);
    return {
        factory,
        token0,
        token1,
        fee,
        tickSpacing,
        maxLiquidityPerTick
    };
};

export const getPoolMutables = async (
    signerOrProvider: SignerOrProvider,
    poolAddress: string
): Promise<PoolMutables> => {
    const poolContract: Contract = getPoolContract(
        signerOrProvider,
        poolAddress
    );
    const [liquidity, slot] = await Promise.all([
        poolContract.liquidity(),
        poolContract.slot0()
    ]);
    return {
        liquidity,
        sqrtPriceX96: slot[0],
        tick: slot[1],
        observationIndex: slot[2],
        observationCardinality: slot[3],
        observationCardinalityNext: slot[4],
        feeProtocol: slot[5],
        unlocked: slot[6]
    };
};

export const getPosition = (
    pool: Pool,
    numOfLowerTicks: number,
    numOfUpperTicks: number,
    liquidity?: number | string
): Position => { 
    return new Position({
        pool: pool,
        liquidity: liquidity ? liquidity : 1,
        tickLower: nearestUsableTick(
            pool.tickCurrent,
            pool.tickSpacing
        ) - pool.tickSpacing * numOfLowerTicks,
        tickUpper: nearestUsableTick(
            pool.tickCurrent,
            pool.tickSpacing
        ) + pool.tickSpacing * numOfUpperTicks
    });
};

export const getNumOfTokens = async (
    signerOrProvider: SignerOrProvider,
    address: string
): Promise<number> => {
    const uniPositionManager: Contract = getPositionManagerContract(
        signerOrProvider
    );
    const numOfTokens: BigNumber = 
        await uniPositionManager.balanceOf(address);
    return numOfTokens.toNumber();
};

export const getTokenIdByIndex = async (
    signerOrProvider: SignerOrProvider,
    index: number,
    address: string
): Promise<number> => {
    const uniPositionManager: Contract = getPositionManagerContract(
        signerOrProvider
    );
    const tokenId: BigNumber = 
        await uniPositionManager.tokenOfOwnerByIndex(
            address,
            index
    );
    return tokenId.toNumber();
};

export const getTokenIdFromLastIndex = async (
    signerOrProvider: SignerOrProvider,
    address: string
): Promise<number> => {
    const num: number = await getNumOfTokens(
        signerOrProvider,
        address
    ); 
    return await getTokenIdByIndex(
        signerOrProvider,
        num - 1,
        address
    );
};

export const getTokenPrice = async (
    signerOrProvider: SignerOrProvider,
    token: Token
): Promise<number> => {
    switch (token.chainId) {
        case MATIC_CHAIN_ID:
            return await getMaticTokenPrice(
                signerOrProvider,
                token
            );
        default:
            throw new Error("Unknown Chain");
    }
};

export const priceOf = async (
    signerOrProvider: SignerOrProvider,
    spotPoolAdress: string,
    token0: Token,
    token1: Token,
    token?: Token,
    decimals?: number
): Promise<number> => {
    const pool: Pool = await getPool(
        signerOrProvider,
        spotPoolAdress,
        token0,
        token1
    );
    return Number.parseFloat(
        pool.priceOf(
            token ? token : token1
        )
        .toFixed(
            decimals ? decimals : 2
        )
    );
};
