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

const ChainLine = (step) => {
  const lines = [];
  const edges = step.lines;
  const linePosition = [];
  const edgeTypes = [];
  for (const edge of edges) {
    const from = edge.from;
    const to = edge.to;
    const edgeType = edge.edge_type;
    if (from.cell === to.cell && from.value == to.value) {
      continue;
    }
    const fromPos = pmPosition(from.cell, from.value);
    const toPos = pmPosition(to.cell, to.value);
    edgeTypes.push(edgeType);
    linePosition.push([fromPos, toPos]);
  }

  const deletePosition = step.remove_candidates.map((cand) => {
    const pos = pmPosition(cand.cell, cand.value);
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

const ChainStep = (step) => {
  const edges = step.lines;
  const stepNodes = [];
  let lastEnd = null;
  for (let i = 0; i < edges.length; i++) {
    const edge = edges[i];
    const from = edge.from;
    const to = edge.to;
    if (!lastEnd || lastEnd.cell != from.cell || lastEnd.value != from.value) {
      if (edge.edge_type === "Strong") {
        stepNodes.push(
          <StepNode
            key={`chain-start-${i}`}
            index={from.cell}
            text={from.value}
            color={StepFinColor}
          />,
        );
      } else {
        stepNodes.push(
          <StepNode
            key={`chain-start-${i}`}
            index={from.cell}
            text={from.value}
            color={StepHintValueColor}
          />,
        );
      }
    }
    lastEnd = to;
    if (edge.edge_type === "Strong") {
      stepNodes.push(
        <StepNode
          key={`chain-end-${i}`}
          index={to.cell}
          text={to.value}
          color={StepHintValueColor}
        />,
      );
    } else {
      stepNodes.push(
        <StepNode
          key={`chain-end-${i}`}
          index={to.cell}
          text={to.value}
          color={StepFinColor}
        />,
      );
    }
  }

  const remove_candidates = step.remove_candidates;
  const deleteNodes = remove_candidates.map((cand) => {
    return (
      <StepNode
        key={`delete-${cand.cell}${cand.value}`}
        index={cand.cell}
        text={cand.value}
        color={`#${cand.color.toString(16)}`}
      />
    );
  });
  const chainLines = ChainLine(step);

  const allNodes = stepNodes.concat(deleteNodes).concat(chainLines);
  return <>{allNodes}</>;
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
    if (
      stepName === "Remote Pair" ||
      stepName === "AIC Type1" ||
      stepName === "AIC Type2" ||
      stepName === "X-Chain" ||
      stepName === "XY-Chain" ||
      stepName === "DisContinuous Nice Loop" ||
      stepName === "Continuous Nice Loop"
    ) {
      let chainStep = ChainStep(nextStep);
      return chainStep;
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
