import { Suspense } from "react";
import PlayComponent from "./PlayComponent";
export default function Play() {
  return (
    <Suspense>
      <PlayComponent />
    </Suspense>
  );
}
