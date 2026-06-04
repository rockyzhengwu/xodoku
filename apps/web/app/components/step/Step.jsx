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
  cellPosition,
} from "../../lib/constant.js";
import {
  calculateChainLinePosition,
  calculateBizer,
  hasChainNodePositioninLine,
  hasCandidateInLine,
  isIntersect,
} from "../../lib/sudoku_util.js";

const hexColor = (color) => `#${color.toString(16).padStart(6, "0")}`;

const StepNode = ({ index, text, color }) => {
  const [x, y] = pmPosition(index, text);
  return (
    <>
      <g className="pointer-events-none">
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

const StructuredChainStep = (step) => {
  const nodeById = new Map(step.chain_nodes.map((node) => [node.id, node]));
  const nodeCenter = (node) => {
    const positions = node.candidates.map((cand) => pmPosition(cand.cell, cand.value));
    const sum = positions.reduce(
      ([x, y], [nextX, nextY]) => [x + nextX, y + nextY],
      [0, 0],
    );
    return [sum[0] / positions.length, sum[1] / positions.length];
  };
  const highlighted = step.chain_nodes.flatMap((node, nodeIndex) =>
    node.candidates.map((cand, candidateIndex) => (
      <StepNode
        key={`structured-${node.id}-${cand.cell}-${cand.value}-${candidateIndex}`}
        index={cand.cell}
        text={cand.value}
        color={
          node.kind === "Als"
            ? AlsColor(nodeIndex)
            : `#${cand.color.toString(16).padStart(6, "0")}`
        }
      />
    )),
  );
  const lines = step.chain_edges.map((edge, index) => {
    const from = nodeCenter(nodeById.get(edge.from));
    const to = nodeCenter(nodeById.get(edge.to));
    return (
      <path
        d={calculateChainLinePosition(from, to)}
        key={`structured-line-${index}`}
        stroke="red"
        strokeWidth="2"
        fill="none"
        strokeDasharray={edge.edge_type === "Weak" ? "10" : ""}
        markerEnd="url(#arrowhead)"
      />
    );
  });
  const removed = step.remove_candidates.map((cand) => (
    <StepNode
      key={`structured-delete-${cand.cell}-${cand.value}`}
      index={cand.cell}
      text={cand.value}
      color={`#${cand.color.toString(16).padStart(6, "0")}`}
    />
  ));
  return <>{highlighted.concat(removed).concat(lines)}</>;
};

const BoardOverlays = (step) => {
  const houses = (step.house_overlays ?? []).map((overlay, index) => {
    const house = overlay.house;
    let x = 20;
    let y = 20;
    let width = 900;
    let height = 100;
    if (house >= 9 && house < 18) {
      x += (house - 9) * 100;
      width = 100;
      height = 900;
    } else if (house >= 18) {
      x += ((house - 18) % 3) * 300;
      y += Math.floor((house - 18) / 3) * 300;
      width = 300;
      height = 300;
    } else {
      y += house * 100;
    }
    return (
      <rect
        key={`house-overlay-${house}-${overlay.role}-${index}`}
        x={x}
        y={y}
        width={width}
        height={height}
        fill={hexColor(overlay.color)}
        opacity="0.08"
        className="pointer-events-none"
      />
    );
  });
  const cells = (step.cell_overlays ?? []).map((overlay, index) => {
    const [x, y] = cellPosition(overlay.cell);
    return (
      <rect
        key={`cell-overlay-${overlay.cell}-${overlay.role}-${index}`}
        x={x}
        y={y}
        width="100"
        height="100"
        fill={hexColor(overlay.color)}
        opacity="0.13"
        className="pointer-events-none"
      />
    );
  });
  return <>{houses.concat(cells)}</>;
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
        message: nextStep.search_truncated
          ? "Expert search reached its budget limit"
          : "Can't find next step",
      });
      return false;
    }
    const overlays = BoardOverlays(nextStep);
    if (nextStep.chain_nodes?.length) {
      return (
        <>
          {overlays}
          {StructuredChainStep(nextStep)}
        </>
      );
    }
    if (
      stepName === "Remote Pair" ||
      stepName === "AIC Type1" ||
      stepName === "AIC Type2" ||
      stepName === "Turbot Fish" ||
      stepName === "X-Chain" ||
      stepName === "XY-Chain" ||
      stepName === "DisContinuous Nice Loop" ||
      stepName === "Continuous Nice Loop"
    ) {
      let chainStep = ChainStep(nextStep);
      return (
        <>
          {overlays}
          {chainStep}
        </>
      );
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

    return (
      <>
        {overlays}
        {setNodes}
        {removeNodes}
        {highlightNotes}
      </>
    );
  };

  return (
    <>
      <StepLayer />
    </>
  );
}
// test data: 9.318.5...6..93.......7.3.97.....8....48379....1.....75.8.6.......31..8...9.587.2
