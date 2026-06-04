import "./globals.css";

import "@mantine/core/styles.css";
import "@mantine/dropzone/styles.css";
import "@mantine/notifications/styles.css";

import { MantineProvider, createTheme } from "@mantine/core";
import { Notifications } from "@mantine/notifications";
import { ModalsProvider } from "@mantine/modals";
import Script from "next/script";

import AppShellComponent from "./components/AppShellComponent";

const siteUrl = "https://xodoku.com";

export const metadata = {
  metadataBase: new URL(siteUrl),
  title: {
    default: "Xodoku - Free Sudoku Scanner, Solver and Generator",
    template: "%s | Xodoku",
  },
  description:
    "Scan Sudoku from images, generate puzzles, and get human-style solving hints in your browser.",
  keywords:
    "sudoku scanner, sudoku solver, sudoku generator, sudoku hints, hodoku web",
  authors: [{ name: "Xodoku" }],
  applicationName: "Xodoku",
  category: "games",
  robots:
    "index, follow, max-image-preview:large, max-snippet:-1, max-video-preview:-1",
  manifest: "/manifest.webmanifest",
  alternates: {
    canonical: "/",
  },
  openGraph: {
    title: "Xodoku - Sudoku Scanner, Solver and Generator",
    description:
      "Scan Sudoku from images, generate puzzles, and get human-style solving hints in your browser.",
    type: "website",
    url: siteUrl,
    siteName: "Xodoku",
    images: [
      {
        url: "/images/og.png",
        width: 1200,
        height: 630,
        alt: "Xodoku Sudoku scanner, solver and generator",
      },
    ],
  },
  twitter: {
    card: "summary_large_image",
    title: "Xodoku - Sudoku Scanner, Solver and Generator",
    description:
      "Scan Sudoku from images, generate puzzles, and get human-style solving hints in your browser.",
    images: ["/images/og.png"],
  },
  icons: {
    icon: [
      { url: "/favicon.ico" },
      { url: "/images/logo.png", sizes: "32x32", type: "image/png" },
      { url: "/images/logo.png", sizes: "16x16", type: "image/png" },
    ],
    apple: [{ url: "/images/logo.png", sizes: "180x180" }],
  },
  appleWebApp: {
    capable: true,
    title: "Xodoku",
    statusBarStyle: "default",
  },
  formatDetection: {
    telephone: false,
  },
};

export const viewport = {
  width: "device-width",
  initialScale: 1,
};

const theme = createTheme({
  headings: { fontFamily: "Inter, sans-serif" },
  primaryColor: "teal",
});

export default function RootLayout({ children }) {
  return (
    <html lang="en">
      <head>
        <Script
          async
          strategy="afterInteractive"
          src="https://www.googletagmanager.com/gtag/js?id=G-XLT80HH0XV"
        />
        <Script id="structured-data" type="application/ld+json">
          {JSON.stringify({
            "@context": "https://schema.org",
            "@type": "WebApplication",
            name: "Xodoku",
            url: siteUrl,
            applicationCategory: "GameApplication",
            operatingSystem: "Web",
            inLanguage: "en",
            offers: {
              "@type": "Offer",
              price: "0",
              priceCurrency: "USD",
            },
            description:
              "Scan Sudoku from images, generate puzzles, and get human-style solving hints in your browser.",
          })}
        </Script>
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
