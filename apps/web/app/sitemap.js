import fs from "fs";
import path from "path";
import { fileURLToPath } from "url";

export const dynamic = "force-static";

const siteUrl = "https://xodoku.com";
const staticLastModified = new Date("2026-06-04");
const appDirectory = path.dirname(fileURLToPath(import.meta.url));
const notesDirectory = path.join(appDirectory, "..", "notes");

function staticRoute(pathname, priority, changeFrequency = "monthly") {
  return {
    url: `${siteUrl}${pathname}`,
    lastModified: staticLastModified,
    changeFrequency,
    priority,
  };
}

export default function sitemap() {
  const techniqueRoutes = fs
    .readdirSync(notesDirectory)
    .filter((fileName) => fileName.endsWith(".md"))
    .sort()
    .map((fileName) => {
      const id = fileName.replace(/\.md$/, "");
      const fullPath = path.join(notesDirectory, fileName);
      return {
        url: `${siteUrl}/techniques/${id}`,
        lastModified: fs.statSync(fullPath).mtime,
        changeFrequency: "monthly",
        priority: id === "introduction" ? 0.8 : 0.7,
      };
    });

  return [
    staticRoute("", 1, "weekly"),
    staticRoute("/scanner", 0.9, "monthly"),
    staticRoute("/techniques", 0.9, "monthly"),
    staticRoute("/privacy", 0.2, "yearly"),
    ...techniqueRoutes,
  ];
}
