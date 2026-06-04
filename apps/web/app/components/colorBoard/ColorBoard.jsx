import { colorPosition, colorPositionMobile } from "../../lib/constant.js";

import { useMediaQuery } from "@mantine/hooks";
const lineClass = "fill-none stroke-[var(--control-border-color)] stroke-1";

export default function ColorBoard({ onBoardClick }) {
  const isMobile = useMediaQuery("(max-width: 768px)");

  const handleClick = (event, colorCode) => {
    event.stopPropagation();
    onBoardClick(colorCode);
  };
  const deskTopLayout = () => {
    return (
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
          width="100"
          height="100"
          className={lineClass}
        ></rect>
        {colorPosition.map((item, index) => {
          return (
            <rect
              key={index}
              x={item.x}
              y={item.y}
              height="100"
              width="100"
              fill={item.color}
              rx="12"
              ry="12"
              onClick={(event) => handleClick(event, item.code)}
              className="cursor-pointer transition-opacity hover:opacity-80"
            ></rect>
          );
        })}
        {[0,100,200,300].map((n) => <path key={`h-${n}`} d={`M 0 ${n} h 300 ${n}`} className={lineClass} />)}
        {[0,100,200,300].map((n) => <path key={`v-${n}`} d={`M ${n} 0 v ${n} 300`} className={lineClass} />)}
      </svg>
    );
  };

  const mobileLayout = () => {
    return (
      <svg
        viewBox="0 0 900 100"
        version="1.1"
        xmlns="http://www.w3.org/2000/svg"
        width="100%"
        className="overflow-hidden rounded-xl border border-[var(--control-border-color)] bg-[var(--control-background-color)]"
      >
        <rect
          x="0"
          y="0"
          width="900"
          height="100"
          className={lineClass}
        ></rect>
        {colorPositionMobile.map((item, index) => {
          return (
            <rect
              key={index}
              x={item.x}
              y={item.y}
              height="100"
              width="100"
              fill={item.color}
              rx="12"
              ry="12"
              onClick={(event) => handleClick(event, item.code)}
              className="cursor-pointer transition-opacity hover:opacity-80"
            ></rect>
          );
        })}

        {[0,100,200,300,400,500,600,700,800,900].map((x) => <path key={x} d={`M ${x} 0 v ${x} 100`} className={lineClass} />)}
      </svg>
    );
  };
  const Mobile = mobileLayout();
  const Desktop = deskTopLayout();

  return <>{isMobile ? Mobile : Desktop}</>;
}
