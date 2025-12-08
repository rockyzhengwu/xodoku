"use client";

import styles from "./PmBoard.module.css";
import { pmPositionOffset } from "../../lib/constant.js";
import { useMediaQuery } from "@mantine/hooks";

function DigitNumber({ digit, x, y, onDigitClick }) {
  const handleClick = (event) => {
    event.stopPropagation();
    onDigitClick(digit);
  };
  const tx = x + pmPositionOffset[digit][0];
  const ty = y + pmPositionOffset[digit][1];
  return (
    <>
      <g onClick={handleClick}>
        <rect
          x={x}
          y={y}
          width={100}
          height={100}
          className={styles.digitBg}
        ></rect>
        <text
          x={tx}
          y={ty}
          className={styles.digit}
          fontSize="36"
          textAnchor="middle"
          dominantBaseline="central"
        >
          {digit}
        </text>
        <rect
          x={x}
          y={y}
          width={100}
          height={100}
          className={styles.digitCover}
        ></rect>
      </g>
    </>
  );
}
const deskTopLayout = (onBoardClick) => {
  return (
    <>
      <svg
        viewBox="0 0 300 300"
        version="1.1"
        xmlns="http://www.w3.org/2000/svg"
        width="100%"
      >
        <rect
          x="0"
          y="0"
          width="300"
          height="300"
          className={styles.digitLine}
        ></rect>
        {Array.from({ length: 9 }, (_, i) => i).map((d) => {
          return (
            <DigitNumber
              key={d + 1}
              digit={d + 1}
              x={Math.floor(d % 3) * 100}
              y={Math.floor(d / 3) * 100}
              onDigitClick={onBoardClick}
            />
          );
        })}
        <path d="M 0 0 h 300 0" className={styles.digitLine}></path>
        <path d="M 0 100 h 300 100" className={styles.digitLine}></path>
        <path d="M 0 200 h 300 200" className={styles.digitLine}></path>
        <path d="M 0 300 h 300 300" className={styles.digitLine}></path>
        <path d="M 0 0 v 1 300" className={styles.digitLine}></path>
        <path d="M 100 0 v 100 300" className={styles.digitLine}></path>
        <path d="M 200 0 v 200 300" className={styles.digitLine}></path>
        <path d="M 300 0 v 300 300" className={styles.digitLine}></path>
      </svg>
    </>
  );
};

const mobileLayout = (onBoardClick) => {
  return (
    <svg
      viewBox="0 0 900 100"
      version="1.1"
      xmlns="http://www.w3.org/2000/svg"
      width="100%"
    >
      {Array.from({ length: 9 }, (_, i) => i).map((d) => {
        return (
          <DigitNumber
            key={d + 1}
            digit={`${d + 1}`}
            x={Math.floor(d * 100)}
            y={0}
            onClick={onBoardClick}
          />
        );
      })}
      <rect
        x="0"
        y="0"
        width="900"
        height="100"
        className={styles.digitLine}
      ></rect>
      <path d="M 0 0 h 900 0" className={styles.digitLine}></path>
      <path d="M 0 98 h 900 98" className={styles.digitLine}></path>
      <path d="M 100 0 v 100 100" className={styles.digitLine}></path>
      <path d="M 200 0 v 200 100" className={styles.digitLine}></path>
      <path d="M 300 0 v 300 100" className={styles.digitLine}></path>
      <path d="M 400 0 v 400 100" className={styles.digitLine}></path>
      <path d="M 500 0 v 500 100" className={styles.digitLine}></path>
      <path d="M 600 0 v 600 100" className={styles.digitLine}></path>
      <path d="M 700 0 v 700 100" className={styles.digitLine}></path>
      <path d="M 800 0 v 800 100" className={styles.digitLine}></path>
      <path d="M 900 0 v 900 100" className={styles.digitLine}></path>
    </svg>
  );
};

export default function PmBoard({ onBoardClick }) {
  const desktop = deskTopLayout(onBoardClick);
  const mobil = mobileLayout(onBoardClick);
  const isMobile = useMediaQuery("(max-width: 768px)");
  return <>{isMobile ? mobil : desktop}</>;
}
