"use client";

import { IconSun, IconMoon } from "@tabler/icons-react";
import {
  useMantineColorScheme,
  Button,
  useComputedColorScheme,
} from "@mantine/core";
import { useState, useEffect } from "react";

export default function ThemeToggleButton() {
  const { setColorScheme } = useMantineColorScheme();
  const computedColorScheme = useComputedColorScheme();
  const [mounted, setMounted] = useState(false);

  useEffect(() => {
    setMounted(true);
  }, []);

  if (!mounted) {
    return null;
  }

  const toggleTheme = () => {
    setColorScheme(computedColorScheme === "light" ? "dark" : "light");
  };

  return (
    <>
      <Button
        onClick={toggleTheme}
        leftSection={
          computedColorScheme === "dark" ? (
            <IconSun size="1rem" />
          ) : (
            <IconMoon size="1rem" />
          )
        }
        fullWidth
        variant="default"
      >
        {computedColorScheme === "dark" ? (
          <span>Light Mode</span>
        ) : (
          <span>Dark Mode</span>
        )}
      </Button>
    </>
  );
}
