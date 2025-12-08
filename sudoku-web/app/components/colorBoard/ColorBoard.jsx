import { colorPosition, colorPositionMobile } from "../../lib/constant.js";

import { useMediaQuery } from "@mantine/hooks";
import styles from "./ColorBoard.module.css";

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
        className="digit"
      >
        <rect
          x="0"
          y="0"
          width="100"
          height="100"
          className="digit-line"
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
              onClick={(event) => handleClick(event, item.code)}
            ></rect>
          );
        })}
        <path d="M 0 0 h 300 0" className={styles.DigitLine}></path>
        <path d="M 0 100 h 300 100" className={styles.DigitLine}></path>
        <path d="M 0 200 h 300 200" className={styles.DigitLine}></path>
        <path d="M 0 300 h 300 300" className={styles.DigitLine}></path>
        <path d="M 0 0 v 1 300" className={styles.DigitLine}></path>
        <path d="M 100 0 v 100 300" className={styles.DigitLine}></path>
        <path d="M 200 0 v 200 300" className={styles.DigitLine}></path>
        <path d="M 300 0 v 300 300" className={styles.DigitLine}></path>
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
        className="digit"
      >
        <rect
          x="0"
          y="0"
          width="900"
          height="100"
          className="digit-line"
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
              onClick={(event) => handleClick(event, item.code)}
            ></rect>
          );
        })}

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
  const Mobile = mobileLayout();
  const Desktop = deskTopLayout();

  return <>{isMobile ? Mobile : Desktop}</>;
}
