"use client";

import sytles from "./Footer.module.css";
import { Anchor } from "@mantine/core";

export default function Footer() {
  return (
    <>
      <div className={sytles.footer}>
        <Anchor href="/privacy" target="_blank">
          privacy
        </Anchor>
        <span> @2025 xodoku All rights reserved.</span>
      </div>
    </>
  );
}
