"use client";

import { useAtom } from "jotai";
import { nextStepAtom } from "../../atom/PlayerAtoms.js";
import Player from "../../components/player/Player.jsx";

import styles from "./Home.module.css";

export default function Home() {
  const [nextStep, setNextStep] = useAtom(nextStepAtom);

  return (
    <>
      <div className={styles.player}>
        <Player />
      </div>
      <div
        className={styles.explainer}
        dangerouslySetInnerHTML={{
          __html: nextStep ? nextStep.explain : "",
        }}
      ></div>
    </>
  );
}
