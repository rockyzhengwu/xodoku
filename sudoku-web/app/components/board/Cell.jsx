"use client";

import { pmPositionOffset } from "../../lib/constant";

import styles from "./Cell.module.css";

export default function Cell({ cell, onMouseDown, onMouseOver, autoPms }) {
  const handleClick = (event) => {
    event.stopPropagation();
  };

  const coverClass = (() => {
    if (!cell.isValidDigit) {
      return styles.cellCoverError;
    }
    if (cell.isSelected) {
      return styles.cellCoverSelected;
    }

    if (cell.isSelectedBuddy) {
      return styles.cellCoverSelectedBuddy;
    }
    if (cell.isSelectedSame) {
      return styles.cellCoverSelectedSame;
    }
    return styles.cellCover;
  })();

  const textDigit = () => {
    if (cell.digit === "0") {
      return null;
    }

    return (
      <text
        x={cell.x + 50}
        y={cell.y + 50}
        fontSize="92"
        textAnchor="middle"
        dominantBaseline="central"
        className={cell.isGiven ? styles.digitGiven : styles.digitNotGiven}
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
          key={pm}
          x={cell.x + ox}
          y={cell.y + oy}
          textAnchor="middle"
          dominantBaseline="central"
          fontSize={28}
          className={styles.pmDigit}
        >
          {pm}
        </text>
      );
    });
    return pmText;
  };

  const textDigitLayer = textDigit();
  const pmDigitLayer = pmDigit();

  const handleMouseDown = (event) => {
    event.stopPropagation();
    onMouseDown({ index: cell.index });
  };

  const handleMouseOver = (event) => {
    event.stopPropagation();
    console.log("mouseMove:", event.currentTarget);
    onMouseOver(event);
  };

  const bgClass = (() => {
    switch (cell.color) {
      case 0:
        return styles.cellBg0;
      case 1:
        return styles.cellBg1;
      case 2:
        return styles.cellBg2;
      case 3:
        return styles.cellBg3;
      case 4:
        return styles.cellBg4;
      case 5:
        return styles.cellBg5;
      case 6:
        return styles.cellBg6;
      case 7:
        return styles.cellBg7;
      case 8:
        return styles.cellBg8;
      case 9:
        return styles.cellBg9;
    }
  })();

  return (
    <>
      <g
        onMouseDown={handleMouseDown}
        onClick={handleClick}
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
