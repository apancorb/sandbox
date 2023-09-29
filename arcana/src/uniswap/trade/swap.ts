import { 
    BigNumber
} from "ethers";
import {
    CurrencyAmount,
    Percent,
    TradeType as UniTradeType,
    Token
} from "@uniswap/sdk-core";
import { 
    AlphaRouter,
    AlphaRouterConfig,
    SwapRoute,
    SwapOptions
} from '@uniswap/smart-order-router'
import { 
    Pool 
} from "@uniswap/v3-sdk";

import {
    Protocol,
    SummaryMetrics,
    SwapMetrics,
    Trader,
    TradeType
} from "./types";
import {
    TransactionReceipt,
    balanceOf,
    msToHMS,
    priceDiff as _priceDiff,
    sendAndWaitForTransaction
} from "../../commons";
import {
    SWAP_ROUTER_ADDRESS,
    priceOf as _priceOf,
    getTokenPrice
} from "../../uniswap";

export const swap = async (
    t: Trader,
    type: TradeType,
    start: Date = new Date()
): Promise<void> => {
    // swap tokens
    let tokenIn: Token, tokenOut: Token;
    if (TradeType.BUY === type) {
        [tokenIn, tokenOut] = [t.base, t.target];
    } else {
        [tokenIn, tokenOut] = [t.target, t.base];
    }
    // swap
    for (let i: number = 0; i < t.config.tries; i++) {
        try {
            const route: SwapRoute =
                await swapData(t, tokenIn, tokenOut);
            const txr: TransactionReceipt =
                await swapTx(t, route);
            const metrics: SwapMetrics =
                await swapMetrics(t, type, tokenIn, tokenOut, txr, i, start);
            if (TradeType.BUY === type) {
                t.metrics.buy = metrics;
            } else {
                t.metrics.sell = swapSummary(t, metrics);
            }
            return;
        } catch (e) {
            t.log ?
                t.log.info("Swap:", e) :
                console.log("Swap:", e);
        }
    }
    // swap failed
    t.metrics.meta.fail = true;
    if (TradeType.BUY === type) {
        t.metrics.buy.fail = true;
    } else {
        t.metrics.sell.fail = true;
    }
    throw new Error("Swap Fatal Error");
};

const swapData = async (
    t: Trader,
    tokenIn: Token,
    tokenOut: Token
): Promise<SwapRoute> => {
    const amount: CurrencyAmount<Token> = 
        CurrencyAmount.fromRawAmount(
            tokenIn,
            (
                await balanceOf(
                    t.net.provider,
                    tokenIn.address,
                    t.net.address
                )
            )
            .toString()
    );
    if (amount.equalTo(0)) {
        throw new Error("Amount to swap is 0");
    }
    const router = new AlphaRouter({
        chainId: t.net.chainId,
        provider: t.net.provider
    });
    const swapConfig: SwapOptions = {
        recipient: t.net.recipient ?
            t.net.recipient :
            t.net.address,
        slippageTolerance: new Percent(
            t.config.slippageTolerance,
            100
        ),
        deadline: Date.now() + 1800
    };
    // assuming the spot pool is the
    // biggest of its pair type by TVL
    const partialRoutingConfig: Partial<AlphaRouterConfig> = {
        protocols: [ Protocol.V3 ],
        maxSwapsPerPath: 1,
        maxSplits: 1,
        v3PoolSelection: {
            topN: 1,
            topNDirectSwaps: 1,
            topNTokenInOut: 0,
            topNSecondHop: 0,
            topNWithBaseToken: 0,
            topNWithEachBaseToken: 0
        }
    };
    const route: SwapRoute = await router.route(
        amount,
        tokenOut,
        UniTradeType.EXACT_INPUT,
        swapConfig,
        partialRoutingConfig
    );
    if (route.methodParameters) {
        return route;
    } else {
        throw new Error("Route Error");
    }
};

const swapTx = async (
    t: Trader,
    route: SwapRoute
): Promise<TransactionReceipt> => {
    return await sendAndWaitForTransaction(
        t.net.signer,
        t.config.confirmations,
        t.config.timeout,
        t.config.speedUp,
        {
            chainId: t.net.chainId,
            from: t.net.address,
            to: SWAP_ROUTER_ADDRESS,
            value: BigNumber.from(route.methodParameters.value),
            nonce: await t.net.signer.getTransactionCount(),
            data: route.methodParameters.calldata,
            gasLimit: BigNumber.from(t.config.gasLimit),
            // increase gas to be minted faster
            gasPrice: BigNumber.from(route.gasPriceWei).mul(2)
        }
    );
};

const swapMetrics = async (
    t: Trader,
    type: TradeType,
    tokenIn: Token,
    tokenOut: Token,
    txr: TransactionReceipt,
    errors: number,
    start: Date,
    end: Date = new Date()
): Promise<SwapMetrics> => {
    const amountInData: string = txr.logs.find(
        log => log.topics.length === 3
            && BigNumber.from(log.topics[1]).eq(t.net.address)
            && BigNumber.from(log.topics[2]).eq(t.poolAddr)
    ).data;
    const amountOutData: string = txr.logs.find(
        log => log.topics.length === 3
            && BigNumber.from(log.topics[1]).eq(t.poolAddr)
            && BigNumber.from(log.topics[2]).eq(t.net.address)
    ).data;
    const swapData: string = txr.logs.find(
        log => log.topics.length === 3
            && BigNumber.from(log.topics[1]).eq(SWAP_ROUTER_ADDRESS)
            && BigNumber.from(log.topics[2]).eq(t.net.address)
    ).data;
    const amountIn: number = Number.parseFloat(
        CurrencyAmount.fromRawAmount(
            tokenIn,
            BigNumber.from(
                amountInData
            ).toString()
        ).toFixed(tokenIn.decimals)
    );
    const amountOut: number = Number.parseFloat(
        CurrencyAmount.fromRawAmount(
            tokenOut,
            BigNumber.from(
                amountOutData
            ).toString()
        ).toFixed(tokenOut.decimals)
    );
    const sqrtPriceX96: string = BigNumber.from(
        "0x" + swapData.substring(2 + (2 * 64), 2 + (3 * 64))
    ).toString();
    const liquidity: string = BigNumber.from(
        "0x" + swapData.substring(2 + (3 * 64), 2 + (4 * 64))
    ).toString();
    const tick: number = Number.parseInt(
        BigNumber.from(
            "0x" + swapData.substring(2 + (4 * 64))
        ).toString()
    );
    console.log(amountIn, amountOut, sqrtPriceX96, liquidity, tick);
    const price$: number = Number.parseFloat(
        new Pool(
            tokenIn,
            tokenOut,
            500,
            sqrtPriceX96,
            liquidity,
            tick
        )
        .priceOf(t.target)
        .toFixed(2)
    );
    const amountIn$: number = Number.parseFloat(
        TradeType.BUY === type ?
            amountIn.toFixed(2) :
            (amountIn * price$).toFixed(2)
    );
    const amountOut$: number = Number.parseFloat(
        TradeType.BUY === type ?
            (amountOut * price$).toFixed(2) :
            amountOut.toFixed(2)
    );
    const amountDiff$: number = Number.parseFloat(
        (amountOut$ - amountIn$).toFixed(2)
    );
    const gasUsed: number = Number.parseFloat(
        CurrencyAmount.fromRawAmount(
            t.net.coin, txr.effectiveGasPrice.mul(
                txr.gasUsed
            ).toString()
        ).toFixed(t.net.coin.decimals)
    );
    const gasUsed$: number = Number.parseFloat(
        (
            await getTokenPrice(
                t.net.provider, t.net.coin
            ) * gasUsed
        ).toFixed(2)
    );
    const time: string = msToHMS(
        end.getTime() - start.getTime()
    );
    return {
        tx: txr.transactionHash,
        type: type,
        tokenIn: tokenIn.symbol,
        tokenOut: tokenOut.symbol,
        price$: price$,
        amountIn: amountIn,
        amountOut: amountOut,
        amountIn$: amountIn$,
        amountOut$: amountOut$,
        amountDiff$: amountDiff$,
        gasUsed: gasUsed,
        gasUsed$: gasUsed$,
        errors: errors,
        start: start,
        end: end,
        time: time 
    };
};

const swapSummary = (
    t: Trader,
    sell: SwapMetrics
): SwapMetrics & SummaryMetrics => {
    if (!t.metrics.buy) {
        return null;
    }
    const price0$: number = 
        t.metrics.buy.price$;
    const price1$: number = 
        sell.price$;
    const diff: number = Number.parseFloat(
        _priceDiff(price0$, price1$).toFixed(2)
    );
    const totalGasUsed$: number = Number.parseFloat(
        (t.metrics.buy.gasUsed$ + sell.gasUsed$).toFixed(2)
    );
    const totalAmountDiff$: number = Number.parseFloat(
        (t.metrics.buy.amountDiff$ + sell.amountDiff$).toFixed(2)
    );
    const gain$: number = Number.parseFloat(
        (sell.amountOut$ - t.metrics.buy.amountIn$).toFixed(2)
    );
    const net$: number = Number.parseFloat(
        (gain$ - totalGasUsed$ + totalAmountDiff$).toFixed(2)
    );
    const totalTime: string = msToHMS(
        sell.end.getTime()
        - t.metrics.buy.start.getTime()
    );
    const summary: SummaryMetrics = {
        price0$: price0$,
        price1$: price1$,
        diff: diff,
        totalGasUsed$: totalGasUsed$,
        totalAmountDiff$: totalAmountDiff$,
        gain$: gain$,
        net$: net$,
        totalTime: totalTime
    };
    return {
        ...sell,
        ...summary
    };
};
