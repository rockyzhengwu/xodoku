"use client";

import { useState } from "react";
import { useDisclosure } from "@mantine/hooks";
import { Modal, Button, Group, Loader, Select, Stack, Text, Tooltip } from "@mantine/core";

export default function RandomGenerateDialog({ generation, onCancel, onInput }) {
  const [opened, { open, close, toggle }] = useDisclosure(false);
  const [difficulty, setDifficulty] = useState("Easy");

  const handleClose = () => {
    if (!generation.running) {
      close();
    }
  };

  const handleChange = (value) => {
    setDifficulty(value);
  };

  const handleGenerate = async () => {
    if (await onInput(difficulty)) {
      close();
    }
  };

  return (
    <>
      <Tooltip label="Generate a random sudoku">
        <Button onClick={toggle} size="sm" radius="md">
          Generate
        </Button>
      </Tooltip>
      <Modal
        opened={opened}
        onClose={handleClose}
        centered
        size="md"
        title="Generate Sudoku"
        closeOnClickOutside={!generation.running}
        closeOnEscape={!generation.running}
        withCloseButton={!generation.running}
      >
        <Stack>
          <Select
            label="Select Sudoku difficulty level"
            placeholder="Select difficutly level"
            data={["Easy", "Medium", "Hard", "Unfair", "Extreme"]}
            value={difficulty}
            onChange={handleChange}
            disabled={generation.running}
          />
          {generation.running ? (
            <Stack align="center" gap="xs">
              <Loader />
              <Text>
                Generating {generation.difficulty} puzzle: {generation.elapsedSeconds}s /{" "}
                {generation.budgetSeconds ?? "..."}s
              </Text>
              <Text c="dimmed" size="sm">
                Hard puzzles can take longer. You can cancel without changing the current board.
              </Text>
            </Stack>
          ) : null}
          <Group justify="center">
            {generation.running ? (
              <Button variant="default" onClick={onCancel}>
                Cancel
              </Button>
            ) : (
              <Button onClick={handleGenerate}>Generate</Button>
            )}
          </Group>
        </Stack>
      </Modal>
    </>
  );
}
