"use client";

import { useAtom } from "jotai";
import { notifications } from "@mantine/notifications";
import { nextStepAtom, sudokuCellsAtom } from "../../atom/PlayerAtoms.js";
import {
  StepHintValueColor,
  StepDeleteCandidateColor,
  StepFinColor,
  pmPosition,
  pmBackgroundCircleSize,
  AlsColor,
  StepEndoFinColor,
} from "../../lib/constant.js";
import {
  calculateChainLinePosition,
  calculateBizer,
  hasChainNodePositioninLine,
  hasCandidateInLine,
  isIntersect,
} from "../../lib/sudoku_util.js";

import styles from "./Step.module.css";

const StepNode = ({ index, text, color }) => {
  const [x, y] = pmPosition(index, text);
  return (
    <>
      <g className={styles.stepNode}>
        <circle cx={x} cy={y} r={pmBackgroundCircleSize} fill={color} />
        <text
          x={x}
          y={y}
          textAnchor="middle"
          dominantBaseline="central"
          fontSize={28}
        >
          {text}
        </text>
      </g>
    </>
  );
};

const SingleValueStep = (step) => {
  let candidate = step.set_values[0];
  const index = candidate.cell;
  const pm = candidate.value;
  return <StepNode index={index} text={pm} color={StepHintValueColor} />;
};

const ChainLine = (step) => {
  const chain = step.chain;
  const lines = [];
  const linePosition = [];
  const edgeTypes = [];
  for (const node of chain) {
    const from = node.link.from;
    const to = node.link.to;
    if (from.cell === to.cell) {
      continue;
    }
    const fromPos = pmPosition(from.cell, from.candidate);
    const toPos = pmPosition(to.cell, to.candidate);
    edgeTypes.push(node.inference_type);
    linePosition.push([fromPos, toPos]);
  }

  const deletePosition = step.delete_candidates.map((cand) => {
    const pos = pmPosition(cand[0], cand[1]);
    return pos;
  });

  for (let i = 0; i < linePosition.length; i++) {
    const inference_type = edgeTypes[i];
    const fromPos = linePosition[i][0];
    const toPos = linePosition[i][1];
    let d = "";
    if (
      hasChainNodePositioninLine(fromPos, toPos, linePosition) ||
      hasCandidateInLine(fromPos, toPos, deletePosition)
    ) {
      d = calculateBizer(fromPos, toPos);
    } else {
      d = calculateChainLinePosition(fromPos, toPos);
    }
    lines.push(
      <path
        d={d}
        key={`line-${i}`}
        stroke="red"
        strokeWidth="2"
        fill="none"
        strokeDasharray={inference_type === "Weak" ? "10" : ""}
        markerEnd="url(#arrowhead)"
      ></path>,
    );
  }
  return lines;
};

export default function Step() {
  const [nextStep, setNextStep] = useAtom(nextStepAtom);
  const [sudokuCells, setSudokuCells] = useAtom(sudokuCellsAtom);

  if (!nextStep) {
    return false;
  }

  const StepLayer = () => {
    const stepName = nextStep.name;
    if (stepName === "Nothing") {
      setNextStep(null);
      notifications.show({
        color: "red",
        title: "no hint",
        message: "Can't find next step",
      });
      return false;
    }
    const setNodes = nextStep.set_values.map((cand, index) => {
      return (
        <StepNode
          key={`delete-${cand.cell}${cand.value}${index}`}
          index={cand.cell}
          text={cand.value}
          color={`#${cand.color.toString(16)}`}
        />
      );
    });
    const removeNodes = nextStep.remove_candidates.map((cand, index) => {
      return (
        <StepNode
          key={`delete-${cand.cell}${cand.value}${index}`}
          index={cand.cell}
          text={cand.value}
          color={`#${cand.color.toString(16)}`}
        />
      );
    });
    const highlightNotes = nextStep.highlight_candidates.map((cand, index) => {
      return (
        <StepNode
          key={`delete-${cand.cell}${cand.value}${index}`}
          index={cand.cell}
          text={cand.value}
          color={`#${cand.color.toString(16)}`}
        />
      );
    });

    return <>{setNodes.concat(removeNodes).concat(highlightNotes)}</>;
  };

  return (
    <>
      <StepLayer />
    </>
  );
}
// test data: 9.318.5...6..93.......7.3.97.....8....48379....1.....75.8.6.......31..8...9.587.2
