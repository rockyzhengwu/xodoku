import "./globals.css";

import "@mantine/core/styles.css";
import "@mantine/core/styles/baseline.css";
import "@mantine/core/styles/default-css-variables.css";
import "@mantine/core/styles/global.css";

import "@mantine/dropzone/styles.css";
import "@mantine/notifications/styles.css";

import { MantineProvider, createTheme } from "@mantine/core";
import { Notifications } from "@mantine/notifications";
import { ModalsProvider } from "@mantine/modals";

import AppShellComponent from "./components/AppShellComponent";

export const metadata = {
  title: {
    default: "Xodoku - Free Sudoku Scanner, Generator And Player",
    template: "%s | Xodoku",
  },
  description:
    "Scan sudoku from image, play and generate sudoku for free. sudoku solver like hodoku . hodoku web online.",
  keywords:
    "sudoku scanner, sudoku solver, sudoku player, sudoku generator, hodoku web online",
  authors: [{ name: "xodoku" }],
  robots:
    "index, follow, max-image-preview:large, max-snippet:-1, max-video-preview:-1",
  alternates: {
    canonical: "https://xodoku.com/",
  },
  openGraph: {
    title: "Xodoku - Sudoku Scanner, Player and Generator",
    description:
      "comprehensive web platform with an advanced scanner, powerful solver, and customizable generator to help you master Sudoku.",
    type: "website",
    url: "https://xodoku.com",
    siteName: "Xodoku",
    images: [
      {
        url: "https://xodoku/images/og.png",
      },
    ],
  },
  twitter: {
    card: "summary_large_image",
    site: "xodoku.com",
    title: "Xodoku- Sudoku scanner, player and Generator",
    description: "",
    images: ["https://xodoku.com/images/og.png"],
  },
  icons: {
    icon: [
      { url: "/favicon.ico" },
      { url: "/images/logo.png", sizes: "32x32", type: "image/png" },
      { url: "/images/logo.png", sizes: "16x16", type: "image/png" },
    ],
    apple: [{ url: "/images/logo.png", sizes: "180x180" }],
  },
};

export default function RootLayout({ children }) {
  const theme = createTheme({
    headings: { fontFamily: "Inter, sans-serif" },
    primaryColor: "teal",
  });

  return (
    <html lang="en">
      <head>
        <meta charSet="UTF-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <link
          href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;700&display=swap"
          rel="stylesheet"
        />
        <script
          async
          src="https://www.googletagmanager.com/gtag/js?id=G-XLT80HH0XV"
        ></script>
      </head>
      <body>
        <MantineProvider theme={theme}>
          <ModalsProvider />
          <Notifications />
          <AppShellComponent>{children}</AppShellComponent>
        </MantineProvider>
      </body>
    </html>
  );
}
