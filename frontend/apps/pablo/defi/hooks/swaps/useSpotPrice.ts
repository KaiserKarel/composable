import { useParachainApi } from "substrate-react";
import { DEFAULT_NETWORK_ID } from "@/defi/utils";
import { callbackGate } from "shared";
import { useEffect } from "react";
import { subscribeSpotPrice } from "@/store/swaps/subscribeSpotPrice";

export const useSpotPrice = () => {
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  useEffect(() => {
    const unsub = callbackGate((_api) => {
      return subscribeSpotPrice(_api);
    }, parachainApi);

    return () => {
      try {
        unsub();
      } catch {}
    };
  }, [parachainApi]);
};