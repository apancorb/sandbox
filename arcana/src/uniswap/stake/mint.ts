import { 
    BigNumber,
    Contract,
    Transaction
} from "ethers";
import { 
    CurrencyAmount,
    Fraction,
    Token,
    Percent
} from "@uniswap/sdk-core";
import { 
    AlphaRouter,
    SwapAndAddConfig,
    SwapAndAddOptions,
    SwapToRatioResponse,
    SwapToRatioRoute,
    SwapToRatioStatus,
    SwapOptions
} from "@uniswap/smart-order-router";
import { 
    AddLiquidityOptions,
    Pool,
    Position
} from "@uniswap/v3-sdk";

import {
    MintMetrics,
    Position as PositionData,
    Staker
} from "../stake";
import {
    POSITION_MANAGER_ADDRESS,
    SWAP_ROUTER_ADDRESS,
    getTokenPrice,
    getPool,
    getPosition,
    getPositionManagerContract
} from "../../uniswap";
import {
    TransactionReceipt,
    Log,
    balanceOf,
    getError,
    msToHMS,
    sendAndWaitForTransaction,
    sleep
} from "../../commons";

export const mint = async (
    s: Staker,
    start: Date = new Date()
): Promise<void> => {
    for (let i: number = 0; i < s.config.tries; i++) {
        try {
            const data: SwapToRatioRoute = 
                await mintData(s);
            const txr: TransactionReceipt = 
                await mintTx(s, data);
            s.position = 
                await positionData(s, txr);
            s.metrics.mint =
                await mintMetrics(s, txr, i, start);
            s.log.info(
                JSON.stringify({
                    meta: s.metrics.meta,
                    position: s.position,
                    mint: s.metrics.mint
                })
            );
            return;
        } catch (e) {
            s.log.error(
                JSON.stringify(
                    getError("mint", e as Error)
                )
            );
            await sleep(s.config.maxSleep);
        } finally {
            await sleep(s.config.mimSleep);
        }
    }
    throw new Error("Mint: Fatal Error");
};

const mintData = async (
    s: Staker
): Promise<SwapToRatioRoute> => {
    const router = new AlphaRouter({
        chainId: s.net.chainId,
        provider: s.net.provider
    });
    const token0Balance: CurrencyAmount<Token> = 
        CurrencyAmount.fromRawAmount(
            s.pool.token0,
            (
                await balanceOf(
                    s.net.provider,
                    s.pool.token0.address,
                    s.net.address
                )
            ).sub(10000).toString()
    );
    const token1Balance: CurrencyAmount<Token> = 
        CurrencyAmount.fromRawAmount(
            s.pool.token1,
            (
                await balanceOf(
                    s.net.provider,
                    s.pool.token1.address,
                    s.net.address
                )
            ).sub(10000).toString()
    );
    const pool: Pool = await getPool(
        s.net.provider,
        s.pool.address,
        s.pool.token0,
        s.pool.token1
    );
    const position: Position = getPosition(
        pool,
        s.config.numOfLowerTicks,
        s.config.numOfUpperTicks
    );
    const swapAndAddConfig: SwapAndAddConfig = {
        maxIterations: 10,
        ratioErrorTolerance: new Fraction(
            s.config.slippageTolerance,
            100
        )
    };
    const swapOptions: SwapOptions = {
        recipient: s.net.address,
        slippageTolerance: new Percent(
            s.config.slippageTolerance,
            100
        ),
        deadline: Date.now() + 1800
    };
    const addLiquidityOptions: AddLiquidityOptions = {
        recipient: s.net.address,
        slippageTolerance: new Percent(
            s.config.slippageTolerance,
            100
        ),
        deadline: Date.now() + 1800
    };
    const swapAndAddOptions: SwapAndAddOptions = {
        swapOptions,
        addLiquidityOptions
    };
    const route: SwapToRatioResponse = await router.routeToRatio(
        token0Balance,
        token1Balance,
        position,
        swapAndAddConfig,
        swapAndAddOptions
    );
    if (route.status !== SwapToRatioStatus.SUCCESS) {
        throw new Error("Route Error");
    }
    return route.result;
};

const mintTx = async (
    s: Staker,
    route: SwapToRatioRoute
): Promise<TransactionReceipt> => {
    const tx: Transaction = {
        chainId: s.net.chainId,
        from: s.net.address,
        to: SWAP_ROUTER_ADDRESS,
        value: BigNumber.from(route.methodParameters.value),
        nonce: await s.net.signer.getTransactionCount(),
        data: route.methodParameters.calldata,
        gasLimit: BigNumber.from(s.config.gasLimit),
        gasPrice: BigNumber.from(route.gasPriceWei)
    };
    return await sendAndWaitForTransaction(
        s.net.signer,
        s.config.confirmations,
        s.config.timeout,
        s.config.speedUp,
        tx
    );
};

const positionData = async (
    s: Staker,
    txr: TransactionReceipt
): Promise<PositionData> => {
    const log: Log = txr.logs.find(
        log => log.topics.length === 2
    );
    const tokenId: number = BigNumber.from(
        log.topics[1]
    ).toNumber();
    const manager: Contract = 
        getPositionManagerContract(s.net.provider);
    const slot: any[] = 
        await manager.positions(tokenId);
    const tickLower: number = BigNumber.from(
        slot[5]
    ).toNumber();
    const tickUpper: number = BigNumber.from(
        slot[6]
    ).toNumber();
    const liquidity: string = BigNumber.from(
        slot[7]
    ).toString();
    const url: string = `https://app.uniswap.org/#/pool/`
        + `${tokenId}`
        + `?chain=polygon`;
    return {
        tokenId: tokenId,
        tickLower: tickLower,
        tickUpper: tickUpper,
        liquidity: liquidity,
        url: url
    };
};

const mintMetrics = async (
    s: Staker,
    txr: TransactionReceipt,
    errors: number,
    start: Date,
    end: Date = new Date()
): Promise<MintMetrics> => {
    const [ price0$, price1$ ] = await Promise.all([
        getTokenPrice(
            s.net.provider,
            s.pool.token0
        ),
        getTokenPrice(
            s.net.provider,
            s.pool.token1
        )
    ]);
    const staked0: number = Number.parseFloat(
        CurrencyAmount.fromRawAmount(
            s.pool.token0,
            BigNumber.from(
                "0x" + txr.logs.find(
                    log => log.topics.length == 2
                ).data.substring(2 + 64, 2 + (64 * 2))
            ).toString()
        ).toFixed(s.pool.token0.decimals)
    );
    const staked1: number = Number.parseFloat(
        CurrencyAmount.fromRawAmount(
            s.pool.token1,
            BigNumber.from(
                "0x" + txr.logs.find(
                    log => log.topics.length == 2
                ).data.substring(2 + (64 * 2))
            ).toString()
        ).toFixed(s.pool.token1.decimals)
    );
    const staked0$: number = Number.parseFloat(
        (price0$ * staked0).toFixed(2)
    );
    const staked1$: number = Number.parseFloat(
        (price1$ * staked1).toFixed(2)
    );
    const gas: number = Number.parseFloat(
        CurrencyAmount.fromRawAmount(
            s.net.coin, txr.effectiveGasPrice.mul(
                txr.gasUsed
            ).toString()
        ).toFixed(s.net.coin.decimals)
    );
    const gas$: number = Number.parseFloat(
        (
            await getTokenPrice(
                s.net.provider, s.net.coin
            ) * gas
        ).toFixed(2)
    );
    const time: string = msToHMS(
        end.getTime() - start.getTime()
    );
    return {
        tx: txr.transactionHash,
        price0$: price0$,
        price1$: price1$,
        staked0: staked0,
        staked1: staked1,
        staked0$: staked0$,
        staked1$: staked1$,
        staked$: staked0$ + staked1$,
        gas: gas,
        gas$: gas$,
        errors: errors,
        start: start,
        end: end,
        time: time 
    };
};
