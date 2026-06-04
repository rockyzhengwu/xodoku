import React, { useEffect, useRef, useCallback } from "react";
import { useAtom } from "jotai";
import { timerRuningAtom, usedSecondsAtom } from "../../atom/PlayerAtoms";

import {
  IconPlayerPlayFilled,
  IconPlayerPauseFilled,
} from "@tabler/icons-react";
import { ActionIcon, Tooltip, Text, Group } from "@mantine/core";

const formatTime = (totalSeconds) => {
  const minutes = Math.floor(totalSeconds / 60);
  const seconds = totalSeconds % 60;
  return `${String(minutes).padStart(2, "0")}:${String(seconds).padStart(2, "0")}`;
};

function Timer() {
  const [usedSeconds, setUsedSeconds] = useAtom(usedSecondsAtom);
  const [isRunning, setIsRunning] = useAtom(timerRuningAtom);

  const timerIdRef = useRef(null);

  useEffect(() => {
    if (isRunning) {
      timerIdRef.current = setInterval(() => {
        setUsedSeconds((prevSeconds) => prevSeconds + 1);
      }, 1000);
    }

    return () => {
      if (timerIdRef.current) {
        clearInterval(timerIdRef.current);
        timerIdRef.current = null;
      }
    };
  }, [isRunning, setUsedSeconds]);

  const handleStart = useCallback(() => {
    if (!isRunning) {
      setIsRunning(true);
    }
  }, [isRunning, setIsRunning]);

  const handlePause = useCallback(() => {
    setIsRunning(false);
  }, [setIsRunning]);

  return (
    <Group gap="xs">
      <Text className="font-mono tracking-tight" size="md">{formatTime(usedSeconds)}</Text>
      {isRunning ? (
        <Tooltip label="Pause timer">
          <ActionIcon onClick={handlePause} variant="subtle">
            <IconPlayerPauseFilled />
          </ActionIcon>
        </Tooltip>
      ) : (
        <Tooltip label="Start timer">
          <ActionIcon onClick={handleStart} variant="subtle">
            <IconPlayerPlayFilled />
          </ActionIcon>
        </Tooltip>
      )}
    </Group>
  );
}

export default Timer;
