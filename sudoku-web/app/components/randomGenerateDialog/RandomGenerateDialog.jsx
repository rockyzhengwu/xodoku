"use client";

import { useState } from "react";
import { useDisclosure } from "@mantine/hooks";
import { Modal, Button, Group, Select } from "@mantine/core";

export default function RandomGenerateDialog({ onInput }) {
  const [opened, { open, close, toggle }] = useDisclosure(false);
  const [difficulty, setDifficulty] = useState("");

  const handleClose = () => {
    close();
  };

  const handleChange = (value) => {
    setDifficulty(value);
  };

  const handleGenerate = () => {
    close();
    onInput(difficulty);
  };

  return (
    <>
      <Button onClick={toggle} size="md">
        Generate Sudoku
      </Button>
      <Modal
        opened={opened}
        onClose={handleClose}
        centered
        size="md"
        title="Generate Sudoku"
      >
        <Group justify="center">
          <Select
            label="Select Sudoku difficulty level"
            placeholder="Select difficutly level"
            data={["Easy", "Medium", "Hard", "Unfair", "Extreme"]}
            defaultValue="Easy"
            onChange={handleChange}
          />
          <Button onClick={handleGenerate}>Generate</Button>
        </Group>
      </Modal>
    </>
  );
}
