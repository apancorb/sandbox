import { Pool } from "@uniswap/v3-sdk";

import { Staker, collectAndRemove, mint } from "../stake";
import { getError, sleep } from "../../commons";
import { getPool } from "../../uniswap";

export const stake = async (s: Staker): Promise<void> => {
  try {
    s.log.info("minting...");
    await mint(s);
    s.log.info("providing...");
    await provide(s);
    s.log.info("collect and removing...");
    await collectAndRemove(s);
  } catch (e) {
    s.log.error(JSON.stringify(getError("stake", e as Error)));
  }
};

const provide = async (s: Staker): Promise<void> => {
  try {
    const [tickLower, tickUpper, time] = [
      s.position.tickLower,
      s.position.tickUpper,
      s.config.maxSleep,
    ];
    for (
      let tick: number = await getCurrentTick(s);
      tickLower <= tick && tick <= tickUpper;
      tick = await getCurrentTick(s)
    ) {
      await sleep(time);
    }
  } catch (e) {
    s.log.error(JSON.stringify(getError("provide", e as Error)));
  }
};

const getCurrentTick = async (s: Staker): Promise<number> => {
  const pool: Pool = await getPool(
    s.net.provider,
    s.pool.address,
    s.pool.token0,
    s.pool.token1,
  );
  return pool.tickCurrent;
};
