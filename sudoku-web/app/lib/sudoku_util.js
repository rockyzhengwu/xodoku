export function hasChainNodePositioninLine(start, end, points) {
  const [x1, y1] = start;
  const [x2, y2] = end;

  for (let i = 0; i < points.length; i++) {
    const [p1, p2] = points[i];
    if ((p1[1] === y2 && p1[0] === x2) || (p1[1] === y1 && p1[0] === x1)) {
      continue;
    }
    if (isIntersect(start, end, p1)) {
      return true;
    }
    if ((p2[1] === y2 && p2[0] === x2) || (p2[1] === y1 && p2[0] === x1)) {
      continue;
    }
    if (isIntersect(start, end, p2)) {
      return true;
    }
  }
  return false;
}
export function hasCandidateInLine(start, end, points) {
  for (let i = 0; i < points.length; i++) {
    if (isIntersect(start, end, points[i])) {
      return true;
    }
  }
  return false;
}

export function isIntersect(start, end, middle) {
  const [x1, y1] = start;
  const [x2, y2] = end;
  const [x3, y3] = middle;
  let isInline = false;
  if ((y3 - y1) * (x2 - x1) == (y2 - y1) * (x3 - x1)) {
    isInline = true;
  }
  if (!isInline) {
    return false;
  }
  let isInSegment = false;
  if (
    Math.min(x1, x2) <= x3 &&
    x3 <= Math.max(x1, x2) &&
    Math.min(y1, y2) <= y3 &&
    y3 <= Math.max(y1, y2)
  ) {
    isInSegment = true;
  }

  return isInline && isInSegment;
}

export function calculateChainLinePosition(from, to) {
  const [x1, y1] = from;
  const [x2, y2] = to;
  const r = 12;
  const distance = calculateDistance(x1, y1, x2, y2);
  const dx = x2 - x1;
  const dy = y2 - y1;
  const ux = dx / distance;
  const uy = dy / distance;
  const startx = Math.ceil(x1 + r * ux);
  const starty = Math.ceil(y1 + r * uy);
  const endx = Math.ceil(x2 - r * ux);
  const endy = Math.ceil(y2 - r * uy);
  const d = `M${startx} ${starty} L${endx} ${endy}`;
  return d;
}

export function calculateBizer(from, to) {
  const r = 12;
  const curveLength = 30;
  const [ox1, oy1] = from;
  const [ox2, oy2] = to;
  const dx = ox2 - ox1;
  const dy = oy2 - oy1;
  const theta = Math.atan2(dy, dx);
  const x1 = ox1 + r * Math.cos(theta - Math.PI / 4.0);
  const y1 = oy1 + r * Math.sin(theta - Math.PI / 4.0);
  const x4 = ox2 - r * Math.cos(theta + Math.PI / 4.0);
  const y4 = oy2 - r * Math.sin(theta + Math.PI / 4.0);
  const x2 = x1 + curveLength * Math.cos(theta - Math.PI / 4.0);
  const y2 = y1 + curveLength * Math.sin(theta - Math.PI / 4.0);
  const x3 = x4 - curveLength * Math.cos(theta + Math.PI / 4.0);
  const y3 = y4 - curveLength * Math.sin(theta + Math.PI / 4.0);
  const d = `M ${x1} ${y1} C ${x2} ${y2}, ${x3} ${y3}, ${x4} ${y4}`;
  return d;
}

function calculateDistance(x1, y1, x2, y2) {
  const xDiff = x2 - x1;
  const yDiff = y2 - y1;
  return Math.sqrt(xDiff * xDiff + yDiff * yDiff);
}

export function checkDigitValid(cells, index, digit) {
  if (digit === "0") {
    return true;
  }
  const cell = cells[index];
  for (let i = 0; i < cells.length; i++) {
    if (i === index) {
      continue;
    }
    const oc = cells[i];
    if (oc.row === cell.row || oc.col === cell.col || oc.box === cell.box) {
      if (oc.digit === digit) {
        return false;
      }
    }
  }
  return true;
}

function row(index) {
  return Math.floor(index / 9);
}

function col(index) {
  return index % 9;
}

const BLOCK = [
  0, 0, 0, 1, 1, 1, 2, 2, 2, 0, 0, 0, 1, 1, 1, 2, 2, 2, 0, 0, 0, 1, 1, 1, 2, 2,
  2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 3, 3, 3, 4, 4, 4, 5, 5, 5, 3, 3, 3, 4, 4, 4, 5,
  5, 5, 6, 6, 6, 7, 7, 7, 8, 8, 8, 6, 6, 6, 7, 7, 7, 8, 8, 8, 6, 6, 6, 7, 7, 7,
  8, 8, 8,
];

function block(index) {
  return BLOCK[index];
}

const Buddies = (() => {
  const buddies = [];
  for (let i = 0; i < 81; i++) {
    const budy = [];
    for (let j = 0; j < 81; j++) {
      if (row(i) === row(j) || col(i) == col(j) || block(i) == block(j)) {
        budy.push(j);
      }
    }
    buddies.push(budy);
  }
  return buddies;
})();

export function getBuddies(index) {
  return Buddies[index];
}

export function computePms(cells) {
  for (let index = 0; index < 81; index += 1) {
    const cell = cells[index];
    if (cell.digit !== "0") {
      continue;
    }
    const pms = [];
    const buddies = getBuddies(cell.index);
    for (let cand = 1; cand <= 9; cand += 1) {
      let isValid = true;
      const candidate = cand.toString();
      for (const budy of buddies) {
        if (cells[budy].digit === candidate) {
          isValid = false;
          break;
        }
      }
      if (isValid) {
        pms.push(candidate);
      }
    }
    cells[index].pms = pms;
  }
  return cells;
}

export function formatGridContent(sudokuCells) {
  const lines = [];
  for (let i = 0; i < 9; i++) {
    const line = [];
    for (let j = 0; j < 9; j++) {
      const index = i * 9 + j;
      const cell = sudokuCells[index];
      if (cell.digit === "0") {
        line.push(cell.pms.join(""));
      } else {
        line.push(cell.digit);
      }
    }
    lines.push(line);
  }
  const colLength = [];
  for (let c = 0; c < 9; c++) {
    let maxLength = 0;
    for (let r = 0; r < 9; r++) {
      if (lines[r][c].length > maxLength) {
        maxLength = lines[r][c].length;
      }
    }
    colLength.push(maxLength);
  }
  const newLines = [];
  const blockLength = [];
  for (let r = 0; r < 9; r++) {
    const line = [];
    const block = [];
    for (let c = 0; c < 9; c++) {
      const length = colLength[c];
      const value = lines[r][c].padEnd(length, " ");
      block.push(value);
      if (c === 2 || c === 5 || c == 8) {
        const b = block.join("  ");
        line.push(b);
        if (r === 0) {
          blockLength.push(b.length);
        }
        line.push("|");
        block.length = 0;
      }
    }
    if (r === 0) {
      const first = [];
      for (const bl of blockLength) {
        first.push("-".repeat(bl));
        first.push(".");
      }
      newLines.push(first);
    }
    newLines.push(line);
    if (r == 2 || r == 5) {
      const sep = [];
      for (const bl of blockLength) {
        sep.push("-".repeat(bl));
        sep.push("|");
      }
      newLines.push(sep);
    }
  }
  const last = [];
  for (const bl of blockLength) {
    last.push("-".repeat(bl));
    last.push(".");
  }
  newLines.push(last);
  const rows = [];
  for (let r = 0; r < newLines.length; r++) {
    const row = newLines[r];
    const s = row.join(" ");
    if (r === 0 || r == 12) {
      rows.push(". " + s);
    } else if (r === 4 || r === 8) {
      rows.push(": " + s);
    } else {
      rows.push("| " + s);
    }
  }
  const gridContent = rows.join("\n");
  return gridContent;
}

export function isSolved(cells) {
  const notSolved = cells.filter((cell) => cell.digit === "0");
  return notSolved.length === 0;
}
