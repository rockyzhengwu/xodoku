"use client";

import { pmPositionOffset } from "../../lib/constant.js";
import { useMediaQuery } from "@mantine/hooks";

const lineClass = "fill-transparent stroke-[var(--control-border-color)] stroke-1";

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
          className="fill-[var(--control-background-color)]"
        ></rect>
        <text
          x={tx}
          y={ty}
          className="fill-[var(--control-digit-color)]"
          fontSize="32"
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
          className="cursor-pointer fill-transparent transition-colors hover:fill-teal-400 hover:opacity-20"
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
        className="overflow-hidden rounded-xl border border-[var(--control-border-color)] bg-[var(--control-background-color)]"
      >
        <rect
          x="0"
          y="0"
          width="300"
          height="300"
          className={lineClass}
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
        {[0,100,200,300].map((n) => <path key={`h-${n}`} d={`M 0 ${n} h 300 ${n}`} className={lineClass} />)}
        {[0,100,200,300].map((n) => <path key={`v-${n}`} d={`M ${n} 0 v ${n} 300`} className={lineClass} />)}
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
      className="overflow-hidden rounded-xl border border-[var(--control-border-color)] bg-[var(--control-background-color)]"
    >
      {Array.from({ length: 9 }, (_, i) => i).map((d) => {
        return (
          <DigitNumber
            key={d + 1}
            digit={`${d + 1}`}
            x={Math.floor(d * 100)}
            y={0}
            onDigitClick={onBoardClick}
          />
        );
      })}
      <rect
        x="0"
        y="0"
        width="900"
        height="100"
        className={lineClass}
      ></rect>
      {[0,100,200,300,400,500,600,700,800,900].map((x) => <path key={x} d={`M ${x} 0 v ${x} 100`} className={lineClass} />)}
    </svg>
  );
};

export default function PmBoard({ onBoardClick }) {
  const desktop = deskTopLayout(onBoardClick);
  const mobil = mobileLayout(onBoardClick);
  const isMobile = useMediaQuery("(max-width: 768px)");
  return <>{isMobile ? mobil : desktop}</>;
}
