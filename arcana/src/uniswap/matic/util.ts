import { 
    Token
} from "@uniswap/sdk-core";

import {
    USDC_TOKEN,
    USDT_TOKEN,
    USDC_SPOT_PRICE_POOL_ADDRESS,
    USDT_SPOT_PRICE_POOL_ADDRESS,
    WBTC_SPOT_PRICE_POOL_ADDRESS,
    WETH_SPOT_PRICE_POOL_ADDRESS,
    WMATIC_SPOT_PRICE_POOL_ADDRESS
} from "./matic";
import {
    SignerOrProvider
} from "../../commons";
import {
    priceOf
} from "../../uniswap";

export const getTokenPrice = async (
    signerOrProvider: SignerOrProvider,
    token: Token
): Promise<number> => {
    switch (token.symbol) {
        case "MATIC":
            return await priceOf(
                signerOrProvider,
                WMATIC_SPOT_PRICE_POOL_ADDRESS,
                USDC_TOKEN,
                token
            );
        case "USDC":
            return await priceOf(
                signerOrProvider,
                USDC_SPOT_PRICE_POOL_ADDRESS,
                USDT_TOKEN,
                token
            );
        case "USDT":
            return await priceOf(
                signerOrProvider,
                USDT_SPOT_PRICE_POOL_ADDRESS,
                USDC_TOKEN,
                token
            );
        case "WBTC":
            return await priceOf(
                signerOrProvider,
                WBTC_SPOT_PRICE_POOL_ADDRESS,
                USDC_TOKEN,
                token
            );
        case "WETH":
            return await priceOf(
                signerOrProvider,
                WETH_SPOT_PRICE_POOL_ADDRESS,
                USDC_TOKEN,
                token
            );
        case "WMATIC":
            return await priceOf(
                signerOrProvider,
                WMATIC_SPOT_PRICE_POOL_ADDRESS,
                USDC_TOKEN,
                token
            );
        default:
            throw new Error("Unknown Token");
    }
};
