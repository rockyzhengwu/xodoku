"use client";
import { useState } from "react";
import { AppShell, Burger, Group, NavLink, Title, Image } from "@mantine/core";
import ThemeToggleButton from "./ThemeToggle/ThemeToggle";
import Link from "next/link";
import { IconGoGame, IconScan, IconNotes } from "@tabler/icons-react";
import Footer from "../components/footer/Footer.js";

export default function AppShellComponent({ children }) {
  "use client";
  const [opened, setOpened] = useState(false);
  const [active, setActive] = useState("Home");

  return (
    <AppShell
      header={{ height: 60 }}
      navbar={{
        width: 300,
        breakpoint: "sm",
        collapsed: { mobile: !opened },
      }}
      padding="md"
    >
      <AppShell.Header>
        <Group h="100%" px="md">
          <Burger
            opened={opened}
            onClick={() => setOpened((o) => !o)}
            hiddenFrom="sm"
            size="sm"
          />
          <img src="/favicon-32x32.png" />
          <Link href="/" style={{ textDecoration: "none", color: "inherit" }}>
            <Title order={3}>Xodoku</Title>
          </Link>
        </Group>
      </AppShell.Header>

      <AppShell.Navbar p="md">
        <AppShell.Section grow>
          <NavLink
            leftSection={<IconGoGame size="1rem" stroke={1.5} />}
            label="Player"
            active={active === "Player"}
            onClick={() => {
              setActive("Player");
              setOpened(false);
            }}
            component={Link}
            href="/"
          />
          <NavLink
            leftSection={<IconScan size="1rem" stroke={1.5} />}
            label="Scanner"
            active={active === "Scanner"}
            onClick={() => {
              setActive("Scanner");
              setOpened(false);
            }}
            component={Link}
            href="/scanner"
          />
          <NavLink
            leftSection={<IconNotes size="1rem" stroke={1.5} />}
            label="Solving techniques"
            active={active === "Techniques"}
            onClick={() => {
              setActive("Techniques");
              setOpened(false);
            }}
            component={Link}
            href="/techniques"
          />
        </AppShell.Section>

        <AppShell.Section>
          <ThemeToggleButton />
          <Footer />
        </AppShell.Section>
      </AppShell.Navbar>

      <AppShell.Main>{children}</AppShell.Main>
    </AppShell>
  );
}
