"use client";

import { pmPositionOffset } from "../../lib/constant";

export default function Cell({ cell, onMouseDown, autoPms }) {
  const coverClass = (() => {
    if (!cell.isValidDigit) {
      return "fill-red-500 opacity-30";
    }
    if (cell.isSelected) {
      return "fill-[var(--cell-selected-bg-color)] fill-opacity-30 stroke-[var(--cell-selected-stroke-color)] stroke-[8] hover:fill-[var(--cell-hover-color)]";
    }

    if (cell.isSelectedBuddy) {
      return "fill-[var(--cell-selected-buddy-color)] opacity-30 hover:fill-[var(--cell-hover-color)]";
    }
    if (cell.isSelectedSame) {
      return "fill-[var(--cell-selected-same-bgc)] opacity-30";
    }
    return "fill-transparent hover:fill-[var(--cell-hover-color)] hover:opacity-30";
  })();

  const textDigit = () => {
    if (cell.digit === "0") {
      return null;
    }

    return (
      <text
        data-kind="digit"
        x={cell.x + 50}
        y={cell.y + 50}
        fontSize="92"
        textAnchor="middle"
        dominantBaseline="central"
        className={cell.isGiven ? "fill-[var(--given-digit-color)]" : "fill-[var(--digit-color)]"}
      >
        {cell.digit}
      </text>
    );
  };

  const pmDigit = () => {
    if (cell.digit !== "0") {
      return null;
    }
    if (!autoPms && !cell.userSetPms) {
      return null;
    }
    const pmText = cell.pms.map((pm) => {
      const ox = pmPositionOffset[pm][0];
      const oy = pmPositionOffset[pm][1];
      return (
        <text
          data-kind="pencil-mark"
          key={pm}
          x={cell.x + ox}
          y={cell.y + oy}
          textAnchor="middle"
          dominantBaseline="central"
          fontSize={28}
          className="fill-[var(--pencil-mark-color)]"
        >
          {pm}
        </text>
      );
    });
    return pmText;
  };

  const textDigitLayer = textDigit();
  const pmDigitLayer = pmDigit();

  const handleSelect = (event) => {
    event.stopPropagation();
    onMouseDown({ index: cell.index });
  };

  const bgClasses = [
    "fill-transparent",
    "fill-[var(--digit-cell-bgc-1)]",
    "fill-[var(--digit-cell-bgc-2)]",
    "fill-[var(--digit-cell-bgc-3)]",
    "fill-[var(--digit-cell-bgc-4)]",
    "fill-[var(--digit-cell-bgc-5)]",
    "fill-[var(--digit-cell-bgc-6)]",
    "fill-[var(--digit-cell-bgc-7)]",
    "fill-[var(--digit-cell-bgc-8)]",
    "fill-[var(--digit-cell-bgc-9)]",
  ];
  const bgClass = bgClasses[cell.color] ?? bgClasses[0];

  return (
    <>
      <g
        onClick={handleSelect}
        data-index={cell.index}
      >
        <rect
          x={cell.x}
          y={cell.y}
          width={100}
          height={100}
          className={bgClass}
        ></rect>
        {cell.digit === "0" ? pmDigitLayer : textDigitLayer}
        <rect
          x={cell.x}
          y={cell.y}
          width={100}
          height={100}
          className={coverClass}
        ></rect>
      </g>
    </>
  );
}
