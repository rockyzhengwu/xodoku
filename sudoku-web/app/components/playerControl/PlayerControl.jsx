"use client";

import DigitBoard from "../../components/digitboard/DigitBoard.jsx";
import PmBoard from "../../components/pmboard/PmBoard.jsx";
import ColorBoard from "../../components/colorBoard/ColorBoard.jsx";
import { undoStack, redoStack } from "../player/PlayerHistory.js";
import RandomGenerateDialog from "../../components/randomGenerateDialog/RandomGenerateDialog.jsx";
import Timer from "../../components/timer/Timer.jsx";

import {
  Stack,
  Group,
  ActionIcon,
  Tooltip,
  Switch,
  Textarea,
} from "@mantine/core";
import {
  IconNumber1,
  IconPencil,
  IconColorFilter,
  IconBulbFilled,
  IconArrowBackUp,
  IconArrowForwardUp,
  IconRestore,
  IconBackspace,
  IconPencilDown,
} from "@tabler/icons-react";
import { SegmentedControl, VisuallyHidden } from "@mantine/core";

const iconProps = {
  style: { display: "block" },
  size: 48,
  stroke: 2.0,
};

import {
  sudokuCellsAtom,
  selectedCellAtom,
  resetCells,
  autoPmsAtom,
  modeAtom,
  nextStepAtom,
  sudokuSolutionAtom,
  usedSecondsAtom,
  initCellsSudoku,
  timerRuningAtom,
} from "../../atom/PlayerAtoms.js";
import {
  DeleteCommand,
  SetColorCommand,
  SetDigitCommand,
  SetPmCommand,
  DeleteCandidateCommand,
} from "../player/command.js";
import { cellsValueString } from "../../atom/cell.js";
import useRustWasm from "../../util/rustWasm.js";

import { useAtom } from "jotai";

import styles from "./PlayerControl.module.css";
import { computePms, isSolved } from "../../lib/sudoku_util.js";
import PlayerActions from "../playerActions/PlayerActions.jsx";

const toolTipText = {
  digit: "Enter a digit",
  pms: "Add a pencil-mark ",
  color: "Colour cell background, second click to clear",
  undo: "Undo last change ",
  redo: "Redo last change ",
  restart: "Restart the puzzle",
  delete: "Delete digits, pencil-marks & color highlights",
};

export default function PlayerControl({}) {
  const { loading, error, rust } = useRustWasm("../../wasm/sudoku_wasm.js");
  const [autoPms, setAutoPms] = useAtom(autoPmsAtom);
  const [mode, setMode] = useAtom(modeAtom);
  const [sudokuCells, setSudokuCells] = useAtom(sudokuCellsAtom);
  const [selectedCell] = useAtom(selectedCellAtom);
  const [nextStep, setNextStep] = useAtom(nextStepAtom);
  const [sudokuSolution, setSudokuSolution] = useAtom(sudokuSolutionAtom);
  const [usedSeconds, setUsedSeconds] = useAtom(usedSecondsAtom);

  const [timerRuning, setTimerRuning] = useAtom(timerRuningAtom);

  const gerateRandomSudoku = (difficulty_level) => {
    setUsedSeconds(0);
    if (rust) {
      const sudokuResult = rust.generate_sudoku(difficulty_level);
      const cells = initCellsSudoku(sudokuResult);
      setNextStep(null);
      setSudokuSolution(sudokuResult.solutions);
      setSudokuCells(cells);
      setTimerRuning(true);
    }
  };
  const handleModeChange = (mode) => {
    setMode(mode);
  };
  const handleDigitInput = (digit) => {
    if (selectedCell === null && selectedCell === undefined) {
      return;
    }
    const cell = sudokuCells[selectedCell];
    if (cell.isGiven) {
      return;
    }
    redoStack.clear();
    const newDigit = cell.digit === digit ? "0" : digit;
    const command = new SetDigitCommand(
      cell.index,
      newDigit,
      cell.digit,
      cell.pms,
      cell.isValidDigit,
    );
    command.execute(sudokuCells, setSudokuCells);
    setNextStep(null);
    undoStack.push(command);
  };

  const handlePmInput = (digit) => {
    if (!selectedCell) {
      return;
    }

    redoStack.clear();
    const command = new SetPmCommand(selectedCell, digit);
    command.execute(sudokuCells, setSudokuCells);
    undoStack.push(command);
  };

  const handleColorInput = (code) => {
    if (!selectedCell) {
      return;
    }
    redoStack.clear();
    const cell = sudokuCells[selectedCell];
    const command =
      cell.color !== code
        ? new SetColorCommand(selectedCell, code, cell.color)
        : new SetColorCommand(selectedCell, 0, cell.color);

    command.execute(sudokuCells, setSudokuCells);
    undoStack.push(command);
  };

  const handleReset = () => {
    undoStack.clear();
    redoStack.clear();
    const newCells = resetCells(sudokuCells);
    const newCellsWithPms = computePms(newCells);
    setSudokuCells(newCellsWithPms);
    setNextStep(null);
  };

  const handleUndo = (event) => {
    event.stopPropagation();
    if (undoStack.isEmpty()) {
      return;
    }
    const command = undoStack.pop();
    command.undo(sudokuCells, setSudokuCells);
    redoStack.push(command);
  };

  const handleRedo = (event) => {
    event.stopPropagation();
    if (redoStack.isEmpty()) {
      return;
    }
    const command = redoStack.pop();
    command.execute(sudokuCells, setSudokuCells);
    undoStack.push(command);
  };

  const handleDelete = (event) => {
    event.stopPropagation();
    if (!selectedCell) {
      return;
    }
    const cell = sudokuCells[selectedCell];
    if (cell.isGiven) {
      return;
    }
    const command = new DeleteCommand(
      selectedCell,
      cell.digit,
      cell.pms,
      cell.color,
      cell.isValidDigit,
    );
    command.execute(sudokuCells, setSudokuCells);
    undoStack.push(command);
  };

  const showPms = () => {
    if (!autoPms) {
      const digitStr = cellsValueString(sudokuCells);
      const pms = rust.calc_pms(digitStr);
      const newCells = sudokuCells;
      for (let i = 0; i < 81; i++) {
        if (newCells[i].userSetPms) {
          continue;
        }
        let pm = pms[i];
        if (pm === "") {
          continue;
        }
        newCells[i].pms = pm.split("");
      }
      setSudokuCells(newCells);
    }
  };

  const handleShowPms = (event) => {
    setAutoPms(!autoPms);
    if (sudokuCells.length == 0) {
      return;
    }
    showPms();
  };

  const handleHint = () => {
    if (isSolved(sudokuCells)) {
      return;
    }
    const digitStr = cellsValueString(sudokuCells);
    const pms = sudokuCells.map((cell) => {
      return cell.pms.join("");
    });
    const is_given = sudokuCells.map((cell) => {
      return cell.isGiven;
    });
    const request = { pms: pms, digits: digitStr, is_given: is_given };
    const step = rust.get_next_step(request);
    console.log("NextHint:", step);
    setNextStep(step);
  };

  const handleDoHint = () => {
    if (!nextStep) {
      return;
    }
    redoStack.clear();
    if (nextStep.remove_candidates.length !== 0) {
      const command = new DeleteCandidateCommand(nextStep.remove_candidates);
      command.execute(sudokuCells, setSudokuCells);
      undoStack.push(command);
      setNextStep(null);
      return;
    }
    const stepType = nextStep.name;
    switch (stepType) {
      case "Hidden Single":
      case "Naked Single":
      case "Full House":
        const setCandidate = nextStep.set_values[0];
        const value = setCandidate.value.toString();
        const index = parseInt(setCandidate.cell, 10);
        const cell = sudokuCells[index];
        const command = new SetDigitCommand(
          index,
          value,
          cell.digit,
          cell.pms,
          cell.isValidDigit,
        );
        command.execute(sudokuCells, setSudokuCells);
        undoStack.push(command);
        break;
      default:
        console.log("not implemnt", stepType);
    }

    // TODO dostep
    setNextStep(null);
  };

  const boardLayer = (() => {
    if (mode === "digit") {
      return <DigitBoard onBoardClick={handleDigitInput} />;
    }
    if (mode === "pms") {
      return <PmBoard onBoardClick={handlePmInput} />;
    }
    if (mode === "color") {
      return <ColorBoard onBoardClick={handleColorInput} />;
    }
    return null;
  })();
  const explainContent = () => {
    const html = nextStep ? nextStep.explain : "<div></div>";
    return { __html: html };
  };

  return (
    <>
      <Stack
        onClick={(event) => {
          event.stopPropagation();
        }}
        align="stretch"
        justify="center"
        gap="md"
      >
        <Group direction={"row"} className={styles.topActionBar}>
          <SegmentedControl
            onChange={handleModeChange}
            className={styles.controlButton}
            data={[
              {
                value: "digit",
                label: (
                  <>
                    <Tooltip label={toolTipText["digit"]}>
                      <IconNumber1 {...iconProps} />
                    </Tooltip>
                    <VisuallyHidden>Digit</VisuallyHidden>
                  </>
                ),
              },
              {
                value: "pms",
                label: (
                  <>
                    <Tooltip label={toolTipText["pms"]}>
                      <IconPencil {...iconProps} />
                    </Tooltip>
                    <VisuallyHidden>Pms</VisuallyHidden>
                  </>
                ),
              },
              {
                value: "color",
                label: (
                  <>
                    <Tooltip label={toolTipText["color"]}>
                      <IconColorFilter {...iconProps} />
                    </Tooltip>
                    <VisuallyHidden>Color</VisuallyHidden>
                  </>
                ),
              },
            ]}
          />
          <Tooltip label="next step">
            <ActionIcon onClick={handleHint} variant="subtle" size="xl">
              <IconBulbFilled size={48} />
            </ActionIcon>
          </Tooltip>
          <Tooltip label="do next step">
            <ActionIcon
              onClick={handleDoHint}
              variant="subtle"
              disabled={!nextStep}
              size="xl"
            >
              <IconPencilDown size={48} />
            </ActionIcon>
          </Tooltip>
        </Group>

        <div className={styles["input-digitboard"]}>{boardLayer}</div>

        <Group>
          <Tooltip label={toolTipText["undo"]} position="bottom">
            <ActionIcon onClick={handleUndo} variant="subtle" size="xl">
              <IconArrowBackUp />
            </ActionIcon>
          </Tooltip>

          <Tooltip label={toolTipText["redo"]} position="bottom">
            <ActionIcon onClick={handleRedo} variant="subtle" size="xl">
              <IconArrowForwardUp />
            </ActionIcon>
          </Tooltip>

          <Tooltip label={toolTipText["delete"]} position="bottom">
            <ActionIcon onClick={handleDelete} variant="subtle" size="xl">
              <IconBackspace />
            </ActionIcon>
          </Tooltip>

          <Tooltip label={toolTipText["restart"]} position="bottom">
            <ActionIcon onClick={handleReset} variant="subtle" size="xl">
              <IconRestore />
            </ActionIcon>
          </Tooltip>
          <Switch
            checked={autoPms}
            label="pms"
            onChange={handleShowPms}
            labelPosition="left"
          />
        </Group>
        <Group>
          <Timer />
          <RandomGenerateDialog onInput={gerateRandomSudoku} />
          <PlayerActions />
        </Group>

        <div className={styles.gridFooter}>
          <div className={styles.timer}></div>
        </div>
      </Stack>
    </>
  );
}
