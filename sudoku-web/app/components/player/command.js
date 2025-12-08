import { checkDigitValid, getBuddies } from "../../lib/sudoku_util.js";

export class SetDigitCommand {
  constructor(index, newDigit, oldDigit, oldPms, oldValid) {
    this.index = index;
    this.newDigit = newDigit;
    this.oldDigit = oldDigit;
    this.oldPms = oldPms;
    this.oldValid = oldValid;
    this.deletePms = [];
  }

  execute(cells, setCells) {
    const buddies = getBuddies(this.index);
    if (this.newDigit !== "0") {
      for (const b of buddies) {
        if (cells[b].pms.includes(this.newDigit)) {
          this.deletePms.push(b);
        }
      }
    }

    const isValid = checkDigitValid(cells, this.index, this.newDigit);
    const updateCells = cells.map((cell) => {
      if (cell.index === this.index) {
        return {
          ...cell,
          digit: this.newDigit,
          pms: [],
          isValidDigit: isValid,
          isSelectedSame: true,
        };
      } else if (this.newDigit !== "0" && cell.digit === this.newDigit) {
        return { ...cell, isSelectedSame: true };
      } else {
        if (this.deletePms.includes(cell.index)) {
          return {
            ...cell,
            isSelectedSame: false,
            pms: cell.pms.filter((v) => v !== this.newDigit),
          };
        } else {
          return { ...cell, isSelectedSame: false };
        }
      }
    });
    setCells(updateCells);
  }

  undo(cells, setCells) {
    const updateCells = cells.map((cell) => {
      if (cell.index === this.index) {
        return {
          ...cell,
          digit: this.oldDigit,
          pms: this.oldPms,
          isValidDigit: this.oldValid,
          isSelectedSame: false,
        };
      } else if (this.oldDigit != "0" && cell.digit === this.oldDigit) {
        return { ...cell, isSelectedSame: true };
      } else {
        if (this.deletePms.includes(cell.index)) {
          return {
            ...cell,
            isSelectedSame: false,
            pms: cell.pms.concat(this.newDigit),
          };
        }
        return { ...cell, isSelectedSame: false };
      }
    });
    setCells(updateCells);
  }
}

export class SetPmCommand {
  constructor(index, pm) {
    this.index = index;
    this.pm = pm;
  }
  execute(cells, setCells) {
    const updateCells = cells.map((cell) => {
      if (cell.index === this.index) {
        if (!cell.userSetPms) {
          const pms = [this.pm];
          return { ...cell, pms: pms, userSetPms: true };
        } else {
          const pms = cell.pms.includes(this.pm)
            ? cell.pms.filter((p) => p !== this.pm)
            : [...cell.pms, this.pm];
          const userSetPms = !(pms.length === 0);
          return { ...cell, pms: pms, userSetPms: userSetPms };
        }
      } else {
        return cell;
      }
    });
    setCells(updateCells);
  }
  undo(cells, setCells) {
    this.execute(cells, setCells);
  }
}

export class SetColorCommand {
  constructor(index, color, oldColor) {
    this.index = index;
    this.color = color;
    this.oldColor = oldColor;
  }
  execute(cells, setCells) {
    const updatedCells = cells.map((cell) => {
      if (cell.index === this.index) {
        return { ...cell, color: this.color };
      } else {
        return cell;
      }
    });
    setCells(updatedCells);
  }

  undo(cells, setCells) {
    const updatedCells = cells.map((cell) => {
      if (cell.index === this.index) {
        return { ...cell, color: this.oldColor };
      } else {
        return cell;
      }
    });
    setCells(updatedCells);
  }
}

export class DeleteCommand {
  constructor(index, oldDigit, oldPms, oldColor, oldValid) {
    this.index = index;
    this.oldDigit = oldDigit;
    this.oldPms = oldPms;
    this.oldColor = oldColor;
    this.oldValid = oldValid;
  }
  execute(cells, setCells) {
    const updateCells = cells.map((cell) => {
      if (cell.index === this.index) {
        return { ...cell, digit: "0", pms: [], color: 0, isValidDigit: true };
      } else {
        return cell;
      }
    });
    setCells(updateCells);
  }
  undo(cells, setCells) {
    const updateCells = cells.map((cell) => {
      if (cell.index === this.index) {
        return {
          ...cell,
          digit: this.oldDigit,
          pms: this.oldPms,
          color: this.oldColor,
          isValidDigit: this.oldValid,
        };
      } else {
        return cell;
      }
    });
    setCells(updateCells);
  }
}
export class DeleteCandidateCommand {
  constructor(deleteCandidates) {
    this.deleteCandidates = deleteCandidates;
  }
  execute(cells, setCells) {
    const newCells = structuredClone(cells);
    for (const candidate of this.deleteCandidates) {
      const index = candidate.cell;
      const pm = candidate.value.toString();
      const cell = newCells[index];
      const newPms = cell.pms.filter((p) => p !== pm);
      cell.pms = newPms;
      newCells[index] = cell;
    }
    setCells(newCells);
  }
  undo(cells, setCells) {
    const newCells = structuredClone(cells);
    for (const candidate of this.deleteCandidates) {
      const index = candidate.cell;
      const pm = candidate.value.toString();
      const cell = newCells[index];
      if (!cell.pms.includes(pm)) {
        const newPms = cell.pms.concat(pm);
        cell.pms = newPms;
        newCells[index] = cell;
      }
    }
    setCells(newCells);
  }
}
