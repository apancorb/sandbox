import { 
    Wallet,
    providers
} from "ethers";

import {
    getLogger
} from "../commons";
import { 
    CHAIN_ID,
    MATIC_TOKEN as COIN,
    WMATIC_TOKEN as TOKEN_0,
    WETH_TOKEN as TOKEN_1
} from "../uniswap/matic";
import { 
    Staker,
    stake
} from "../uniswap/stake";

const INFURA_PROJECT_ID: string = process.env.INFURA_PROJECT_ID;
const INFURA_PROJECT_SECRET: string = process.env.INFURA_PROJECT_SECRET;
const ADDRESS: string = process.env.ADDRESS;
const PRIVATE_KEY: string = process.env.PRIVATE_KEY;
const COLLECTOR_ADDRESS: string = process.env.COLLECTOR_ADDRESS;
const POOL_ADDRESS: string = process.env.POOL_ADDRESS;

const main = async (): Promise<void> => {
    const provider: providers.InfuraProvider = 
        new providers.InfuraProvider(
            CHAIN_ID,
            {
                projectId: INFURA_PROJECT_ID,
                projectSecret: INFURA_PROJECT_SECRET
            }
    );
    const signer: Wallet = new Wallet(
        PRIVATE_KEY,
        provider
    );
    for (;;) {
        const merlin: Staker = {
            net: {
                chainId: CHAIN_ID,
                coin: COIN,
                address: ADDRESS,
                recipient: COLLECTOR_ADDRESS,
                provider: provider,
                signer: signer,
            },
            pool: {
                address: POOL_ADDRESS,
                token0: TOKEN_0,
                token1: TOKEN_1
            },
            config: {
                numOfLowerTicks: 4,
                numOfUpperTicks: 4,
                confirmations: 1,
                timeout: 100000,
                speedUp: true,
                tries: 2,
                gasLimit: 1000000,
                slippageTolerance: 1,
                maxSleep: 1,
                mimSleep: 0.3
            },
            metrics: {
                meta: {
                    chainId: CHAIN_ID,
                    staker: "merlin",
                    address: ADDRESS,
                    pool: POOL_ADDRESS,
                    token0: TOKEN_0.address,
                    token1: TOKEN_1.address,
                }
            },
            log: (
                getLogger("./logs/merlin")
            )
        };
        await stake(merlin);
    }
};

main();
