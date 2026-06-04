const OUT_NUM = 300;
const DETECT_CLASSES = ["sudoku"];
const OCR_CLASSES = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
const INPUT_WIDTH = 640;
const INPUT_HEIGHT = 640;

export function processOutput(output, img_width, img_height, mode = "detect") {
  let boxes = [];
  for (let index = 0; index < OUT_NUM; index++) {
    const pos = index * 6;
    const prob = output[pos + 4];
    const class_id = output[pos + 5];
    if (prob < 0.25) {
      continue;
    }
    const label =
      mode === "detect" ? DETECT_CLASSES[class_id] : OCR_CLASSES[class_id];

    const x1 = (output[pos] / INPUT_WIDTH) * img_width;
    const y1 = (output[pos + 1] / INPUT_HEIGHT) * img_height;
    const x2 = (output[pos + 2] / INPUT_WIDTH) * img_width;
    const y2 = (output[pos + 3] / INPUT_HEIGHT) * img_height;
    boxes.push([x1, y1, x2, y2, label, prob]);
  }
  boxes = boxes.sort((box1, box2) => box2[5] - box1[5]);
  return boxes;
}

export async function prepareInput(buf) {
  return new Promise((resolve) => {
    const img = new Image();
    img.src = URL.createObjectURL(buf);
    img.onload = () => {
      const [img_width, img_height] = [img.width, img.height];
      const canvas = document.createElement("canvas");
      canvas.width = 640;
      canvas.height = 640;
      const context = canvas.getContext("2d");
      context.drawImage(img, 0, 0, 640, 640);
      const imgData = context.getImageData(0, 0, 640, 640);
      const pixels = imgData.data;

      const red = [],
        green = [],
        blue = [];
      for (let index = 0; index < pixels.length; index += 4) {
        red.push(pixels[index] / 255.0);
        green.push(pixels[index + 1] / 255.0);
        blue.push(pixels[index + 2] / 255.0);
      }
      const input = [...red, ...green, ...blue];
      resolve([input, img_width, img_height, img.src]);
    };
  });
}
