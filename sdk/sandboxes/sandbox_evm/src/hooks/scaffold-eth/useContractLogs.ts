import { useEffect, useState } from "react";
import { Address, Log } from "viem";
import { usePublicClient } from "wagmi";

import { useTargetNetwork } from "./useTargetNetwork";

export const useContractLogs = (address: Address) => {
  const [logs, setLogs] = useState<Log[]>([]);
  const { targetNetwork } = useTargetNetwork();
  const client = usePublicClient({ chainId: targetNetwork.id });

  useEffect(() => {
    const fetchLogs = async () => {
      if (!client) return console.error("Client not found");
      try {
        const existingLogs = await client.getLogs({
          address: address,
          fromBlock: BigInt(0),
          toBlock: "latest",
        });
        setLogs(existingLogs);
      } catch (error) {
        console.error("Failed to fetch logs:", error);
      }
    };
    fetchLogs();

    return client?.watchBlockNumber({
      onBlockNumber: async (_blockNumber, prevBlockNumber) => {
        const newLogs = await client.getLogs({
          address: address,
          fromBlock: prevBlockNumber,
          toBlock: "latest",
        });
        setLogs((prevLogs) => [...prevLogs, ...newLogs]);
      },
    });
  }, [address, client]);

  return logs;
};
