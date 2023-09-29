import { Trader, TradeType, swap } from "../trade";
import { priceDiff as _priceDiff } from "../../commons";
import { priceOf as _priceOf } from "../../uniswap";

export const trade = async (t: Trader): Promise<void> => {
  const { buyDiff, sellDiff } = t.config;
  let lock: boolean = false;
  let low: number,
    high: number,
    curr: number,
    i: number = 0;
  let type: TradeType = TradeType.BUY;
  curr = low = high = await priceOf(t);
  t.net.provider.on({ address: t.poolAddr }, async () => {
    if (lock) {
      return;
    }
    // hold a lock here
    lock = true;
    curr = await priceOf(t);
    if (TradeType.BUY === type) {
      if (curr < low) {
        low = curr;
      } else if (curr > low && priceDiff(curr, low, buyDiff)) {
        // buy
        await swap(t, type);
        console.log(t.metrics.buy);
        curr = low = high = await priceOf(t);
        type = TradeType.SELL;
      }
    } else {
      if (curr > high) {
        high = curr;
      } else if (curr < high && priceDiff(curr, high, sellDiff)) {
        // sell
        await swap(t, type);
        console.log(t.metrics.sell);
        curr = low = high = await priceOf(t);
        type = TradeType.BUY;
      }
    }
    lock = false;
  });
};

const priceOf = async (t: Trader): Promise<number> => {
  return await _priceOf(t.net.provider, t.poolAddr, t.base, t.target, t.target);
};

const priceDiff = (p0: number, p1: number, target: number): boolean => {
  return _priceDiff(p0, p1, true) >= target;
};
