import { 
    BigNumber,
    Contract
} from "ethers";

import {
    SignerOrProvider
} from "./types";

export const balanceOf = async (
    signerOrProvider: SignerOrProvider,
    tokenAddress: string,
    address: string,
): Promise<BigNumber> => {
    const tokenContract: Contract = new Contract(
        tokenAddress,
        ["function balanceOf(address account) external view returns (uint256)"],
        signerOrProvider
    );
    return await tokenContract.balanceOf(address);
};
