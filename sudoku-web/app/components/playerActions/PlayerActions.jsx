"use client";
import { useState, useRef } from "react";

import {
  ActionIcon,
  Menu,
  Button,
  Stack,
  Text,
  Textarea,
  Title,
} from "@mantine/core";
import {
  IconDotsVertical,
  IconCopy,
  IconPlus,
  IconRocket,
} from "@tabler/icons-react";
import { useClipboard } from "@mantine/hooks";
import { useAtom } from "jotai";
import { modals } from "@mantine/modals";
import {
  sudokuCellsAtom,
  initCellsSudoku,
  timerRuningAtom,
  sudokuSolutionAtom,
  nextStepAtom,
} from "../../atom/PlayerAtoms.js";
import { formatGridContent } from "../../lib/sudoku_util.js";
import useRustWasm from "../../util/rustWasm.js";

import styles from "./PlayerActions.module.css";
import { notifications } from "@mantine/notifications";

const sampleGridExample = `.--------------.-------------.-------------.
| 7   5    46  | 9    8    2 | 46  3    1  |
| 2   14   9   | 6    14   3 | 5   8    7  |
| 13  346  8   | 14   5    7 | 9   26   24 |
:--------------+-------------+-------------:
| 8   12   12  | 5    9    4 | 3   7    6  |
| 6   47   47  | 3    2    1 | 8   9    5  |
| 5   9    3   | 8    7    6 | 14  12   24 |
:--------------+-------------+-------------:
| 13  367  5   | 147  46   9 | 2   146  8  |
| 9   8    16  | 2    146  5 | 7   46   3  |
| 4   267  267 | 17   3    8 | 16  5    9  |
'--------------'-------------'-------------'`;

export default function PlayerActions() {
  const [sudokuCells, setSudokuCells] = useAtom(sudokuCellsAtom);
  const clipboard = useClipboard({ timeout: 500 });
  const { loading, error, rust } = useRustWasm("../wasm/sudoku_wasm.js");
  const [timerRuning, setTimerRuning] = useAtom(timerRuningAtom);
  const [sudokuSolution, setSudokuSolution] = useAtom(sudokuSolutionAtom);
  const [nextStep, setNextStep] = useAtom(nextStepAtom);

  const inputRef = useRef(null);

  const handleCopy = async () => {
    if (sudokuCells.length === 0) {
      return;
    }
    const originalDigit = sudokuCells.map((cell) => {
      if (cell.isGiven) {
        return cell.digit;
      } else {
        return "0";
      }
    });
    const originalLine = originalDigit.join("").replaceAll("0", ".");
    const shareUrl = `https://www.xodoku.com/play?s=${originalLine}`;

    const digits = sudokuCells.map((cell) => {
      return cell.digit;
    });
    const singleLine = digits.join("").replaceAll("0", ".");

    const gridContent = formatGridContent(sudokuCells);
    const copyContent = (content) => {
      if (!navigator.clipboard && document.execCommand) {
        try {
          const tempTextArea = document.createElement("textarea");
          tempTextArea.value = content;
          document.body.appendChild(tempTextArea);
          tempTextArea.select();
          document.execCommand("copy");
          document.body.removeChild(tempTextArea);
          clipboard.copied = true;
        } catch (err) {
          console.error("Fallback copy failed:", err);
        }
      } else {
        clipboard.copy(content);
      }
    };

    modals.open({
      title: "Copy Sudoku",
      children: (
        <>
          <Stack>
            <p>{shareUrl}</p>
            <Button
              onClick={() => {
                copyContent(shareUrl);
                modals.closeAll();
              }}
            >
              Copy Original URL
            </Button>
            <p>{originalLine}</p>
            <Button
              onClick={() => {
                copyContent(originalLine);
                modals.closeAll();
              }}
            >
              Copy Original Sudoku
            </Button>
            <p>{singleLine}</p>
            <Button
              onClick={() => {
                copyContent(singleLine);
                modals.closeAll();
              }}
            >
              Copy Current State Line Format
            </Button>
            <pre>{gridContent}</pre>
            <Button
              onClick={() => {
                copyContent(gridContent);
                modals.closeAll();
              }}
            >
              Copy Current State Grid Format
            </Button>
          </Stack>
        </>
      ),
      labels: { cancel: "Cancel" },
      onCancel: () => console.log("Cancel"),
      size: "xl",
    });
  };

  const inputSudoku = async () => {
    const text = inputRef.current.value;
    if (text.length === 0) {
      return;
    }

    modals.closeAll();
    try {
      const sudoku = await rust.import_sudoku(text);
      const cells = initCellsSudoku(sudoku);
      setSudokuSolution(sudoku.solutions);
      setSudokuCells(cells);
      setTimerRuning(true);
      setNextStep(null);
    } catch (error) {
      console.log(error);
      notifications.show({
        color: "red",
        title: "Invalid Sudoku",
        message: error,
      });
    }
  };

  const handleImportText = async () => {
    modals.open({
      labels: { cancel: "Cancel" },
      onCancel: () => console.log("Cancel"),
      size: "xl",
      children: (
        <>
          <Text>
            Please input the Sudoku grid as an 81-character string or Grid
            format
          </Text>
          <Title>Single line text</Title>
          <Text>
            <Text fw={700} component="span">
              Empty cells:
            </Text>
            can be represented by either a period (.) or a zero (0).
          </Text>
          <Text>
            <Text fw={700} component="span">
              Example:
            </Text>
            ..2...3...3......767............61..........47..52..6..2.3.49...9...7.8...69.8.1.
          </Text>
          <Title>Grid Format</Title>
          <Text fw={700}>Example:</Text>
          <pre>{sampleGridExample}</pre>
          <Textarea
            data-autofocus
            placeholder="input sudoku text "
            autosize
            minRows={2}
            ref={inputRef}
          />
          <Button
            variant="subtle"
            onClick={inputSudoku}
            className={styles.loadButton}
            size="xl"
          >
            Load
          </Button>
        </>
      ),
    });
  };

  const showSolution = () => {
    if (sudokuSolution.length === 0) {
      return;
    }
    let content = "";
    const lineSplit = "------------------------\n";
    content = content.concat(lineSplit);
    for (let i = 0; i < 9; i++) {
      let line = "|";
      for (let j = 0; j < 9; j++) {
        const pos = i * 9 + j;
        const digit = sudokuSolution[pos];
        line = line.concat(" ", digit.toString());
        if (j === 2 || j === 5) {
          line = line.concat(" |");
        }
      }
      line = line.concat("|");
      content = content.concat(line, "\n");
      if (i === 2 || i == 5) {
        content = content.concat(lineSplit);
      }
    }
    content = content.concat(lineSplit);

    modals.open({
      title: "Solution",
      children: (
        <>
          <pre>{content}</pre>
        </>
      ),
      labels: { cancel: "Cancel" },
      onCancel: () => console.log("Cancel"),
      size: "xl",
    });
  };

  return (
    <>
      <Menu>
        <Menu.Target>
          <ActionIcon size="xl">
            <IconDotsVertical onLoad={() => {}} />
          </ActionIcon>
        </Menu.Target>
        <Menu.Dropdown>
          <Menu.Item leftSection={<IconCopy size={14} />} onClick={handleCopy}>
            Copy
          </Menu.Item>
          <Menu.Item
            leftSection={<IconPlus size={14} />}
            onClick={handleImportText}
          >
            Import from Text
          </Menu.Item>

          <Menu.Item
            leftSection={<IconRocket size={14} />}
            onClick={showSolution}
          >
            Show Solution
          </Menu.Item>
        </Menu.Dropdown>
      </Menu>
    </>
  );
}
