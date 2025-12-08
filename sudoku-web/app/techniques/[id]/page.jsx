import { getPostData } from "../../lib/posts.js";
import styles from "./page.module.css";

export default async function TechniqueNote({ params }) {
  const { id, contentHtml } = await getPostData(params.id);
  return (
    <>
      <div
        dangerouslySetInnerHTML={{ __html: contentHtml }}
        className={styles["post"]}
      />
    </>
  );
}
