"use client";
import { useState } from "react";
import { AppShell, Burger, Group, NavLink, Title } from "@mantine/core";
import ThemeToggleButton from "./ThemeToggle/ThemeToggle";
import Link from "next/link";
import Image from "next/image";
import { usePathname } from "next/navigation";
import { IconGoGame, IconScan, IconNotes } from "@tabler/icons-react";
import Footer from "../components/footer/Footer.js";

export default function AppShellComponent({ children }) {
  const [opened, setOpened] = useState(false);
  const pathname = usePathname();
  const closeNavbar = () => setOpened(false);

  return (
    <AppShell
      header={{ height: 56 }}
      navbar={{
        width: 220,
        breakpoint: "sm",
        collapsed: { mobile: !opened },
      }}
      padding={{ base: "xs", sm: "md", lg: "lg" }}
    >
      <AppShell.Header>
        <Group h="100%" px="md">
          <Burger
            opened={opened}
            onClick={() => setOpened((o) => !o)}
            hiddenFrom="sm"
            size="sm"
          />
          <Image src="/favicon-32x32.png" alt="" width={26} height={26} />
          <Link href="/" style={{ textDecoration: "none", color: "inherit" }}>
            <Title order={3} className="text-2xl tracking-tight">Xodoku</Title>
          </Link>
        </Group>
      </AppShell.Header>

      <AppShell.Navbar p="sm">
        <AppShell.Section grow>
          <NavLink
            leftSection={<IconGoGame size="1rem" stroke={1.5} />}
            label="Player"
            active={pathname === "/" || pathname.startsWith("/play")}
            onClick={closeNavbar}
            component={Link}
            href="/"
          />
          <NavLink
            leftSection={<IconScan size="1rem" stroke={1.5} />}
            label="Scanner"
            active={pathname.startsWith("/scanner")}
            onClick={closeNavbar}
            component={Link}
            href="/scanner"
          />
          <NavLink
            leftSection={<IconNotes size="1rem" stroke={1.5} />}
            label="Solving techniques"
            active={pathname.startsWith("/techniques")}
            onClick={closeNavbar}
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
