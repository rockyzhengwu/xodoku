import { useMediaQuery } from "@mantine/hooks";

const lineClass = "fill-none stroke-[var(--control-border-color)] stroke-1";

function DigitNumber({ digit, x, y, onClick }) {
  const handleClick = (event) => {
    event.preventDefault();
    event.stopPropagation();
    onClick(digit);
  };
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
          className="select-none fill-[var(--control-digit-color)]"
          x={x + 50}
          y={y + 50}
          fontSize="60"
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

const desktopLayout = (onBoardClick) => {
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
        width="300"
        height="300"
        className={lineClass}
      ></rect>
      {Array.from({ length: 9 }, (_, i) => i).map((d) => {
        return (
          <DigitNumber
            key={d + 1}
            digit={`${d + 1}`}
            x={Math.floor(d % 3) * 100}
            y={Math.floor(d / 3) * 100}
            onClick={onBoardClick}
          />
        );
      })}
      <path d="M 0 0 h 300 0" className={lineClass}></path>
      <path d="M 0 100 h 300 100" className={lineClass}></path>
      <path d="M 0 200 h 300 200" className={lineClass}></path>
      <path d="M 0 300 h 300 300" className={lineClass}></path>
      <path d="M 0 0 v 1 300" className={lineClass}></path>
      <path d="M 100 0 v 100 300" className={lineClass}></path>
      <path d="M 200 0 v 200 300" className={lineClass}></path>
      <path d="M 300 0 v 300 300" className={lineClass}></path>
    </svg>
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
            onClick={onBoardClick}
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

export default function DigitBoard({ onBoardClick }) {
  const isMobile = useMediaQuery("(max-width: 768px)");
  const MobileLayer = mobileLayout(onBoardClick);
  const DesktopLayer = desktopLayout(onBoardClick);
  return <>{isMobile ? MobileLayer : DesktopLayer}</>;
}
