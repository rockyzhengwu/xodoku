"use client";

import { Anchor } from "@mantine/core";

export default function Footer() {
  return (
      <div className="mt-4 flex flex-col items-center gap-1 text-center text-xs text-gray-500">
        <Anchor href="/privacy">
          Privacy
        </Anchor>
        <span>&copy; {new Date().getFullYear()} Xodoku</span>
      </div>
  );
}
