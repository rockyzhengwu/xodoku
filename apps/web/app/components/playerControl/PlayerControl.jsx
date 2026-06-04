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
  size: 24,
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
import { useCallback } from "react";
import { computePms, isSolved } from "../../lib/sudoku_util.js";
import PlayerActions from "../playerActions/PlayerActions.jsx";
import useSudokuGenerator from "./useSudokuGenerator.js";

const toolTipText = {
  digit: "Enter a digit",
  pms: "Add a pencil-mark ",
  color: "Colour cell background, second click to clear",
  undo: "Undo last change ",
  redo: "Redo last change ",
  restart: "Restart the puzzle",
  delete: "Delete digits, pencil-marks & color highlights",
};

export default function PlayerControl() {
  const { rust } = useRustWasm("../../wasm/sudoku_wasm.js");
  const [autoPms, setAutoPms] = useAtom(autoPmsAtom);
  const [mode, setMode] = useAtom(modeAtom);
  const [sudokuCells, setSudokuCells] = useAtom(sudokuCellsAtom);
  const [selectedCell] = useAtom(selectedCellAtom);
  const [nextStep, setNextStep] = useAtom(nextStepAtom);
  const [, setSudokuSolution] = useAtom(sudokuSolutionAtom);
  const [, setUsedSeconds] = useAtom(usedSecondsAtom);

  const [, setTimerRuning] = useAtom(timerRuningAtom);
  const onGenerationSuccess = useCallback((sudoku) => {
    setUsedSeconds(0);
    setNextStep(null);
    setSudokuSolution(sudoku.solutions);
    setSudokuCells(initCellsSudoku(sudoku));
    setTimerRuning(true);
  }, [
    setNextStep,
    setSudokuCells,
    setSudokuSolution,
    setTimerRuning,
    setUsedSeconds,
  ]);
  const { generation, generate, cancel } = useSudokuGenerator({ onSuccess: onGenerationSuccess });
  const handleModeChange = (mode) => {
    setMode(mode);
  };
  const handleDigitInput = (digit) => {
    if (selectedCell == null) {
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
    if (selectedCell == null) {
      return;
    }

    redoStack.clear();
    const command = new SetPmCommand(selectedCell, digit);
    command.execute(sudokuCells, setSudokuCells);
    undoStack.push(command);
  };

  const handleColorInput = (code) => {
    if (selectedCell == null) {
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
    if (selectedCell == null) {
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
      const newCells = sudokuCells.map((cell) => ({ ...cell }));
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
    const request = {
      pms: pms,
      digits: digitStr,
      is_given: is_given,
      hint_mode: "expert",
    };
    const step = rust.get_next_step(request);
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
  return (
      <Stack
        data-testid="player-control-panel"
        onClick={(event) => {
          event.stopPropagation();
        }}
        align="stretch"
        gap="md"
      >
        <section className="space-y-2">
          <div className="text-xs font-semibold uppercase tracking-[0.18em] text-[var(--panel-muted-color)]">
            Mode
          </div>
          <Group gap="xs" wrap="nowrap" align="center">
          <SegmentedControl
            size="sm"
            radius="md"
            onChange={handleModeChange}
            value={mode}
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
            <ActionIcon onClick={handleHint} variant="light" size="lg" radius="md">
              <IconBulbFilled size={22} />
            </ActionIcon>
          </Tooltip>
          <Tooltip label="do next step">
            <ActionIcon
              onClick={handleDoHint}
              variant="light"
              disabled={!nextStep}
              size="lg"
              radius="md"
            >
              <IconPencilDown size={22} />
            </ActionIcon>
          </Tooltip>
          </Group>
        </section>

        <section className="space-y-2">
          <div className="text-xs font-semibold uppercase tracking-[0.18em] text-[var(--panel-muted-color)]">
            Input
          </div>
          <div className="w-full">{boardLayer}</div>
        </section>

        <section className="space-y-2">
          <div className="text-xs font-semibold uppercase tracking-[0.18em] text-[var(--panel-muted-color)]">
            Edit
          </div>
          <Group gap="xs">
          <Tooltip label={toolTipText["undo"]} position="bottom">
            <ActionIcon onClick={handleUndo} variant="subtle" size="lg" radius="md">
              <IconArrowBackUp size={21} />
            </ActionIcon>
          </Tooltip>

          <Tooltip label={toolTipText["redo"]} position="bottom">
            <ActionIcon onClick={handleRedo} variant="subtle" size="lg" radius="md">
              <IconArrowForwardUp size={21} />
            </ActionIcon>
          </Tooltip>

          <Tooltip label={toolTipText["delete"]} position="bottom">
            <ActionIcon onClick={handleDelete} variant="subtle" size="lg" radius="md">
              <IconBackspace size={21} />
            </ActionIcon>
          </Tooltip>

          <Tooltip label={toolTipText["restart"]} position="bottom">
            <ActionIcon onClick={handleReset} variant="subtle" size="lg" radius="md">
              <IconRestore size={21} />
            </ActionIcon>
          </Tooltip>
          </Group>
        </section>

        <section className="space-y-3 border-t border-[var(--panel-border-color)] pt-3">
          <Group justify="space-between" align="center">
            <Timer />
          <Switch
            checked={autoPms}
            label="pms"
            onChange={handleShowPms}
            labelPosition="left"
            size="sm"
          />
          </Group>
          <Group justify="space-between" align="center" wrap="nowrap">
          <RandomGenerateDialog
            generation={generation}
            onCancel={cancel}
            onInput={generate}
          />
          <PlayerActions />
          </Group>
        </section>
      </Stack>
  );
}
