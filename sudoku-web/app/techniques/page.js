import Link from "next/link";
import { Container } from "@mantine/core";
import styles from "./page.module.css";

export default async function Techniques() {
  const TechniqueItem = ({ name, href }) => {
    return (
      <>
        {href ? (
          <>
            <Link href={href} className={styles["link"]}>
              {name
                .replace("-", " ")
                .replace(/(^|\s)[a-z]/gi, (l) => l.toUpperCase())}
            </Link>
          </>
        ) : (
          <span>
            {name
              .replace("-", " ")
              .replace(/(^|\s)[a-z]/gi, (l) => l.toUpperCase())}
          </span>
        )}
      </>
    );
  };

  return (
    <>
      <div>
        <h1>Solving techniques</h1>
        <TechniqueItem
          name="introduction"
          href="/techniques/introduction"
        ></TechniqueItem>
        <div className={styles.category}>
          <TechniqueItem name="singles" href="/techniques/singles" />
        </div>

        <div className={styles.category}>
          <TechniqueItem
            name="intersections"
            href="/techniques/intersections"
          />
        </div>

        <div className={styles.category}>
          <TechniqueItem
            name="hidden-subsets"
            href="/techniques/hidden-subsets"
          />
        </div>

        <div className={styles.category}>
          <h3>Naked Subsets</h3>
          <TechniqueItem name="naked-pair" />
          <TechniqueItem name="naked-tripe" />
          <TechniqueItem name="naked-quadrpue" />
        </div>

        <div className={styles.category}>
          <h3>Baic Fish</h3>
          <TechniqueItem name="x-wing" />
          <TechniqueItem name="swordfish" />
          <TechniqueItem name="jellyfish" />
          <TechniqueItem name="squirmbag" />
          <TechniqueItem name="whale" />
          <TechniqueItem name="leviathan" />
        </div>
        <div className={styles.category}>
          <h3>Finned Fish</h3>
          <TechniqueItem name="finned-x-wing" />
          <TechniqueItem name="finned-swordfish" />
          <TechniqueItem name="finned-jellyfish" />
          <TechniqueItem name="finned-squirmbag" />
          <TechniqueItem name="finned-whale" />
          <TechniqueItem name="finned-leviathan" />
        </div>

        <div className={styles.category}>
          <h3>Single Digit pattern</h3>
          <TechniqueItem name="skyscraper" />
          <TechniqueItem name="2-string-kite" />
          <TechniqueItem name="turbot-fish" />
          <TechniqueItem name="empty-rectangle" />
        </div>

        <div className={styles.category}>
          <h3>Uniquess</h3>
          <TechniqueItem name="unique-rectangle type 1" />
          <TechniqueItem name="unique-rectangle type 2" />
          <TechniqueItem name="unique-rectangle type 3" />
          <TechniqueItem name="unique-rectangle type 4" />
          <TechniqueItem name="unique-rectangle type 5" />
          <TechniqueItem name="unique-rectangle type 6" />
          <TechniqueItem name="binary-universal-grave " />
          <TechniqueItem name="hidden-rectangle" />
        </div>

        <div className={styles.category}>
          <h3>Wings</h3>
          <TechniqueItem name="XY-Wing" />
          <TechniqueItem name="XYZ-Wing" />
          <TechniqueItem name="W-wing" />
        </div>
        <div>
          <h3>Chain and Loops</h3>
          <TechniqueItem name="Remote Pair" />
          <TechniqueItem name="X-chain" />
          <TechniqueItem name="XY-Chain" />
          <TechniqueItem name="Nice-loop" />
          <TechniqueItem name="Alternate Inference Chain" />
        </div>
        <div>
          <h3>Others</h3>
          <TechniqueItem name="Sue de Coq" />
        </div>
      </div>
    </>
  );
}
