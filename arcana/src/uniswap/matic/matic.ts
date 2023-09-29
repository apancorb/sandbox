import {
    Token
} from "@uniswap/sdk-core";

// poylgon 

export const CHAIN_ID: number = 137;

// matic

export const MATIC_CONTRACT_ADDRESS: string =
    "0x0000000000000000000000000000000000001010";
export const MATIC_CONTRACT_ABI: string[] = [
    "function balanceOf(address account) external view returns (uint256);",
    "function transfer(address recipient, uint256 amount) external returns (bool);"
];
export const MATIC_TOKEN = new Token(
    CHAIN_ID,
    MATIC_CONTRACT_ADDRESS,
    18,
    "MATIC",
    "Matic (Polygon)"
);

// usdc 

export const USDC_CONTRACT_ADDRESS: string =
    "0x2791bca1f2de4661ed88a30c99a7a9449aa84174";
export const USDC_SPOT_PRICE_POOL_ADDRESS: string =
    "0x3F5228d0e7D75467366be7De2c31D0d098bA2C23";
export const USDC_CONTRACT_ABI: string[] = [
    "function balanceOf(address account) external view returns (uint256);",
    "function transfer(address recipient, uint256 amount) external returns (bool);"
];
export const USDC_TOKEN = new Token(
    CHAIN_ID,
    USDC_CONTRACT_ADDRESS,
    6,
    "USDC",
    "USD Coin"
);

// usdt 

export const USDT_CONTRACT_ADDRESS: string =
    "0xc2132D05D31c914a87C6611C10748AEb04B58e8F";
export const USDT_SPOT_PRICE_POOL_ADDRESS: string =
    "0x3F5228d0e7D75467366be7De2c31D0d098bA2C23";
export const USDT_CONTRACT_ABI: string[] = [
    "function balanceOf(address account) external view returns (uint256);",
    "function transfer(address recipient, uint256 amount) external returns (bool);"
];
export const USDT_TOKEN = new Token(
    CHAIN_ID,
    USDT_CONTRACT_ADDRESS,
    6,
    "USDT",
    "USD Tether"
);

// wbtc

export const WBTC_CONTRACT_ADDRESS: string =
    "0x1BFD67037B42Cf73acF2047067bd4F2C47D9BfD6";
export const WBTC_SPOT_PRICE_POOL_ADDRESS: string =
    "0x847b64f9d3A95e977D157866447a5C0A5dFa0Ee5";
export const WBTC_CONTRACT_ABI: string[] = [
    "function balanceOf(address account) external view returns (uint256);",
    "function transfer(address recipient, uint256 amount) external returns (bool);"
];
export const WBTC_TOKEN = new Token(
    CHAIN_ID,
    WBTC_CONTRACT_ADDRESS, 
    8,
    "WBTC",
    "Wrapped Bitcoin"
);

// weth

export const WETH_CONTRACT_ADDRESS: string =
    "0x7ceb23fd6bc0add59e62ac25578270cff1b9f619";
export const WETH_SPOT_PRICE_POOL_ADDRESS: string =
    "0x45dDa9cb7c25131DF268515131f647d726f50608";
export const WETH_CONTRACT_ABI: string[] = [
    "function balanceOf(address account) external view returns (uint256);",
    "function transfer(address recipient, uint256 amount) external returns (bool);"
];
export const WETH_TOKEN = new Token(
    CHAIN_ID,
    WETH_CONTRACT_ADDRESS, 
    18,
    "WETH",
    "Wrapped Ether"
);

// wmatic

export const WMATIC_CONTRACT_ADDRESS: string =
    "0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270";
export const WMATIC_SPOT_PRICE_POOL_ADDRESS: string =
    "0xA374094527e1673A86dE625aa59517c5dE346d32";
export const WMATIC_CONTRACT_ABI: string[] = [
    "function balanceOf(address account) external view returns (uint256);",
    "function transfer(address recipient, uint256 amount) external returns (bool);"
];
export const WMATIC_TOKEN = new Token(
    CHAIN_ID,
    WMATIC_CONTRACT_ADDRESS,
    18,
    "WMATIC",
    "Wrapped Matic (Polygon)"
);
