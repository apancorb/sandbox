import { Signer, Transaction } from "ethers";

import { TransactionReceipt, TransactionResponse } from "../commons";

export const sendAndWaitForTransaction = async (
  signer: Signer,
  confirmations: number,
  timeout: number,
  speedUp: boolean,
  tx: Transaction,
): Promise<TransactionReceipt> => {
  const txResponse: TransactionResponse = await signer.sendTransaction(tx);
  const txReceipt: TransactionReceipt = await waitForTransaction(
    signer,
    txResponse,
    confirmations,
    timeout,
    speedUp,
  );
  return txReceipt;
};

export const waitForTransaction = async (
  signer: Signer,
  txResponse: TransactionResponse,
  confirmations: number = 1,
  timeout: number = 200000,
  speedUp: boolean = false,
): Promise<TransactionReceipt> => {
  let txReceipt: TransactionReceipt;
  try {
    txReceipt = await signer.provider.waitForTransaction(
      txResponse.hash,
      confirmations,
      timeout,
    );
  } catch (e) {
    const isTimeout: boolean = (e as Error).message.includes("timeout");
    if (isTimeout && speedUp) {
      const stxResponse: TransactionResponse = await speedUpTransaction(
        signer,
        txResponse,
      );
      txReceipt = await signer.provider.waitForTransaction(
        stxResponse.hash,
        confirmations,
        timeout,
      );
    } else {
      throw e;
    }
  }
  return txReceipt;
};

export const speedUpTransaction = async (
  signer: Signer,
  txResponse: TransactionResponse,
): Promise<TransactionResponse> => {
  const tx: Transaction = {
    chainId: txResponse.chainId,
    from: txResponse.from,
    to: txResponse.to,
    value: txResponse.value,
    nonce: txResponse.nonce,
    data: txResponse.data,
    gasLimit: txResponse.gasLimit,
    gasPrice: txResponse.gasPrice,
  };
  if (!tx.gasPrice) {
    tx.gasPrice = await signer.getGasPrice();
  }
  tx.gasPrice = tx.gasPrice.add(tx.gasPrice.div(2));
  return await signer.sendTransaction(tx);
};
