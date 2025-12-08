const undos = [];
const redos = [];

export const undoStack = {
  push(command) {
    undos.push(command);
  },
  pop() {
    return undos.pop();
  },
  clear() {
    undos.length = 0;
  },

  isEmpty() {
    return undos.length === 0;
  },
};

export const redoStack = {
  push(command) {
    redos.push(command);
  },

  pop() {
    return redos.pop();
  },

  clear() {
    redos.length = 0;
  },
  isEmpty() {
    return redos.length === 0;
  },
};
