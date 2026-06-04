import { remark } from "remark";
import html from "remark-html";
import path from "path";
import { fileURLToPath } from "url";
import * as fs from "fs";
import matter from "gray-matter";

const appDirectory = path.dirname(fileURLToPath(import.meta.url));
const postsDirectory = path.join(appDirectory, "..", "..", "notes");

const imageAltText = {
  full_house: "Full house Sudoku example",
  hidden_single: "Hidden single Sudoku example",
  naked_single: "Naked single Sudoku example",
  hidden_pair: "Hidden pair Sudoku example",
  hidden_tripe: "Hidden triple Sudoku example",
  hidden_quadruple: "Hidden quadruple Sudoku example",
  locked_candidates_type_1: "Pointing locked candidates Sudoku example",
  locked_candidates_type_2: "Claiming locked candidates Sudoku example",
  sudoku_terminology: "Sudoku grid terminology diagram",
};

function enhanceImages() {
  return (tree) => {
    const visit = (node) => {
      if (!node || typeof node !== "object") return;

      if (node.type === "image") {
        const fileName = path.basename(node.url, path.extname(node.url));

        if (node.url.startsWith("../images/")) {
          node.url = node.url.replace("../images/", "/images/");
        }

        if (!node.alt || node.alt.toLowerCase() === "example") {
          node.alt = imageAltText[fileName] ?? "Sudoku solving technique example";
        }

        node.data = {
          ...node.data,
          hProperties: {
            ...node.data?.hProperties,
            loading: "lazy",
            decoding: "async",
          },
        };
      }

      if (Array.isArray(node.children)) {
        node.children.forEach(visit);
      }
    };

    visit(tree);
  };
}

function addImageAttributes(contentHtml) {
  return contentHtml.replaceAll(
    "<img ",
    '<img loading="lazy" decoding="async" ',
  );
}

export function postExists(id) {
  return fs.existsSync(path.join(postsDirectory, `${id}.md`));
}

export async function getPostData(id) {
  const fullPath = path.join(postsDirectory, `${id}.md`);
  const fileContents = fs.readFileSync(fullPath, "utf8");

  // Use gray-matter to parse the post metadata section
  const matterResult = matter(fileContents);

  // Use remark to convert markdown into HTML string
  const processedContent = await remark()
    .use(enhanceImages)
    .use(html)
    .process(matterResult.content);
  const contentHtml = addImageAttributes(processedContent.toString());

  // Combine the data with the id and contentHtml
  return {
    id,
    contentHtml,
    ...matterResult.data,
  };
}

export function getAllPostIds() {
  const fileNames = fs.readdirSync(postsDirectory);

  return fileNames
    .filter((fileName) => fileName.endsWith(".md"))
    .map((fileName) => ({
      id: fileName.replace(/\.md$/, ""),
    }));
}
