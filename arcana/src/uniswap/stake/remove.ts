import { BigNumber, Contract, Transaction } from "ethers";
import { CurrencyAmount, Percent } from "@uniswap/sdk-core";
import {
  CollectOptions,
  MethodParameters,
  NonfungiblePositionManager,
  Pool,
  Position,
  RemoveLiquidityOptions,
} from "@uniswap/v3-sdk";

import { RemoveMetrics, Staker, SummaryMetrics } from "../stake";
import { POSITION_MANAGER_ADDRESS, getPosition, getTokenPrice } from "../util";
import {
  TransactionReceipt,
  getError,
  msToHMS,
  sendAndWaitForTransaction,
  sleep,
} from "../../commons";
import { getPositionManagerContract, getPool } from "../../uniswap";

export const collectAndRemove = async (s: Staker): Promise<void> => {
  await remove(s, true);
  s.position.liquidity = await getPositionLiquidity(s);
  await remove(s);
  s.metrics.summary = summaryMetrics(s);
  s.log.info(
    JSON.stringify({
      meta: s.metrics.meta,
      position: s.position,
      summary: s.metrics.summary,
    }),
  );
};

export const remove = async (
  s: Staker,
  collect: boolean = false,
  start: Date = new Date(),
): Promise<void> => {
  for (let i: number = 0; i < s.config.tries; i++) {
    try {
      const data: MethodParameters = await removeData(s, collect);
      const txr: TransactionReceipt = await removeTx(s, data);
      const metrics: RemoveMetrics = await removeMetrics(s, txr, i, start);
      let obj: any = {
        meta: s.metrics.meta,
        position: s.position,
      };
      if (collect) {
        s.metrics.collect = obj.collect = metrics;
      } else {
        s.metrics.remove = obj.remove = metrics;
      }
      s.log.info(JSON.stringify(obj));
      return;
    } catch (e) {
      s.log.error(
        JSON.stringify(getError(collect ? "collect" : "remove", e as Error)),
      );
      await sleep(s.config.maxSleep);
    } finally {
      await sleep(s.config.mimSleep);
    }
  }
  throw new Error("Remove: Fatal Error");
};

export const removeData = async (
  s: Staker,
  collect: boolean = false,
): Promise<MethodParameters> => {
  const pool: Pool = await getPool(
    s.net.provider,
    s.pool.address,
    s.pool.token0,
    s.pool.token1,
  );
  const position: Position = getPosition(
    pool,
    s.config.numOfLowerTicks,
    s.config.numOfUpperTicks,
    s.position.liquidity,
  );
  const collectOptions: CollectOptions = {
    tokenId: s.position.tokenId,
    expectedCurrencyOwed0: CurrencyAmount.fromRawAmount(s.pool.token0, 0),
    expectedCurrencyOwed1: CurrencyAmount.fromRawAmount(s.pool.token1, 0),
    recipient: collect ? s.net.recipient : s.net.address,
  };
  const removeLiquidityOptions: RemoveLiquidityOptions = {
    tokenId: s.position.tokenId,
    liquidityPercentage: collect ? new Percent(1, 1000000000) : new Percent(1),
    slippageTolerance: new Percent(s.config.slippageTolerance, 100),
    deadline: Date.now() + 1800,
    collectOptions: collectOptions,
  };
  return NonfungiblePositionManager.removeCallParameters(
    position,
    removeLiquidityOptions,
  );
};

const removeTx = async (
  s: Staker,
  data: MethodParameters,
): Promise<TransactionReceipt> => {
  const tx: Transaction = {
    chainId: s.net.chainId,
    from: s.net.address,
    to: POSITION_MANAGER_ADDRESS,
    value: BigNumber.from(data.value),
    nonce: await s.net.signer.getTransactionCount(),
    data: data.calldata,
    gasLimit: BigNumber.from(s.config.gasLimit),
    gasPrice: await s.net.signer.getGasPrice(),
  };
  return await sendAndWaitForTransaction(
    s.net.signer,
    s.config.confirmations,
    s.config.timeout,
    s.config.speedUp,
    tx,
  );
};

const removeMetrics = async (
  s: Staker,
  txr: TransactionReceipt,
  errors: number,
  start: Date,
  end: Date = new Date(),
): Promise<RemoveMetrics> => {
  const [price0$, price1$] = await Promise.all([
    getTokenPrice(s.net.provider, s.pool.token0),
    getTokenPrice(s.net.provider, s.pool.token1),
  ]);
  const data: string = txr.logs[txr.logs.length - 2].data;
  const removedRaw0: string = BigNumber.from(
    "0x" + data.substring(64 + 2, 64 * 2 + 2),
  ).toString();
  const removedRaw1: string = BigNumber.from(
    "0x" + data.substring(64 * 2 + 2),
  ).toString();
  const removed0: number = Number.parseFloat(
    removedRaw0 !== "0"
      ? CurrencyAmount.fromRawAmount(s.pool.token0, removedRaw0).toFixed(
          s.pool.token0.decimals,
        )
      : removedRaw0,
  );
  const removed1: number = Number.parseFloat(
    removedRaw1 !== "0"
      ? CurrencyAmount.fromRawAmount(s.pool.token1, removedRaw1).toFixed(
          s.pool.token1.decimals,
        )
      : removedRaw1,
  );
  const removed0$: number = Number.parseFloat((price0$ * removed0).toFixed(2));
  const removed1$: number = Number.parseFloat((price1$ * removed1).toFixed(2));
  const gas: number = Number.parseFloat(
    CurrencyAmount.fromRawAmount(
      s.net.coin,
      txr.effectiveGasPrice.mul(txr.gasUsed).toString(),
    ).toFixed(s.net.coin.decimals),
  );
  const gas$: number = Number.parseFloat(
    ((await getTokenPrice(s.net.provider, s.net.coin)) * gas).toFixed(2),
  );
  const time: string = msToHMS(end.getTime() - start.getTime());
  return {
    tx: txr.transactionHash,
    price0$: price0$,
    price1$: price1$,
    removed0: removed0,
    removed1: removed1,
    removed0$: removed0$,
    removed1$: removed1$,
    removed$: removed0$ + removed1$,
    gas: gas,
    gas$: gas$,
    errors: errors,
    start: start,
    end: end,
    time: time,
  };
};

const summaryMetrics = (s: Staker): SummaryMetrics => {
  const collected$: number = s.metrics.collect.removed$;
  const impermanent$: number = Number.parseFloat(
    (s.metrics.remove.removed$ - s.metrics.mint.staked$).toFixed(2),
  );
  const gas$: number = Number.parseFloat(
    (
      s.metrics.mint.gas$ +
      s.metrics.collect.gas$ +
      s.metrics.remove.gas$
    ).toFixed(2),
  );
  const net$: number = Number.parseFloat(
    (collected$ + impermanent$ - gas$).toFixed(2),
  );
  const time: string = msToHMS(
    s.metrics.remove.end.getTime() - s.metrics.mint.start.getTime(),
  );
  return {
    collected$: collected$,
    impermanent$: impermanent$,
    gas$: gas$,
    net$: net$,
    time: time,
  };
};

const getPositionLiquidity = async (s: Staker): Promise<string> => {
  const manager: Contract = getPositionManagerContract(s.net.provider);
  const slot: any[] = await manager.positions(s.position.tokenId);
  return BigNumber.from(slot[7]).toString();
};
