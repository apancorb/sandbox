# Arcana

Arcana allows configurable strategies and automatic execution of concentrated 
liquidity pool positions for the uniswap protocol, see https://app.uniswap.org/pools.

The strategies are configured as `actors`, where each actor defines
a strategy that thinks will work best for a given uniswap liquidity
pool.

Right now `arcana` only supports one actor and the polygon chain,
but it can be easily extended to include more actors and chains where
the uniswap protocol is present.
