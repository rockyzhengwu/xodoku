"use client";
import { useState, useEffect } from "react";
import { IconChevronLeft } from "@tabler/icons-react";

import { Group, Container, Text, Button, Alert } from "@mantine/core";
import { Dropzone, IMAGE_MIME_TYPE } from "@mantine/dropzone";
import { IconUpload, IconPhoto, IconX } from "@tabler/icons-react";

import * as ort from "onnxruntime-web";
import Editor from "./editor/Editor.jsx";
import { initCellsFromOcr, editCellsAtom } from "../../atom/EditorAtom.js";
import { useAtom } from "jotai";
import useOnnxModel from "../../util/useOnnxModel.js";
import { prepareInput, processOutput } from "../../lib/yolo.js";
import { ocrPostProcess } from "../../lib/ocr.js";
import { notifications } from "@mantine/notifications";

import styles from "./Scanner.module.css";

export default function Scanner() {
  const [edit, setEdit] = useState(false);
  const [editCells, setEditCells] = useAtom(editCellsAtom);
  const [sudokuImage, setSudokuImage] = useState("");
  const [processing, setProcessing] = useState(false);
  const [noSudokuAlert, setNoSudokuAlert] = useState(false);
  const [inputFile, setInputFile] = useState(null);
  const [cropedImage, setCropedImage] = useState(null);
  const [isDetected, setIsDetected] = useState(false);

  const sessionOptions = {
    executionProviders: ["webgl", "wasm"],
    graphOptimizationLevel: "all",
  };
  const {
    session: detectSession,
    loading: detectLoading,
    error: detectError,
    runInference: detectInference,
  } = useOnnxModel("/detect.onnx", sessionOptions);

  const {
    session: ocrSession,
    loading: ocrLoading,
    error: ocrError,
    runInference: ocrInference,
  } = useOnnxModel("/ocr.onnx");

  useEffect(() => {
    if (detectSession && !detectLoading && !detectError && inputFile) {
      setIsDetected(false);
      detectSudoku(inputFile);
      setIsDetected(true);
      console.log("ONNX model loaded successfully!");
    }
  }, [detectSession, detectLoading, detectError, inputFile]);

  useEffect(() => {
    if (ocrSession && !ocrLoading && !ocrError && isDetected && cropedImage) {
      if (!cropedImage) {
      } else {
        ocrSudoku(cropedImage);
      }
      setProcessing(false);
      console.log("ONNX model ocr successfully");
    }
  }, [ocrSession, ocrLoading, ocrError, isDetected, cropedImage]);

  function cropImage(buf, width, height, box) {
    return new Promise((resolve, reject) => {
      const x1 = box[0];
      const y1 = box[1];
      const x2 = box[2];
      const y2 = box[3];

      const img = new Image();
      const objectURL = URL.createObjectURL(buf);

      img.onload = () => {
        URL.revokeObjectURL(objectURL);
        const canvas = document.createElement("canvas");
        const ctx = canvas.getContext("2d");
        const cropWidth = x2 - x1;
        const cropHeight = y2 - y1;
        if (cropWidth <= 0 || cropHeight <= 0) {
          reject(
            new Error("Invalid crop box: width or height is zero or negative."),
          );
          return;
        }
        canvas.width = cropWidth;
        canvas.height = cropHeight;
        ctx.drawImage(
          img,
          x1,
          y1,
          cropWidth,
          cropHeight,
          0,
          0,
          cropWidth,
          cropHeight,
        );
        const dataURL = canvas.toDataURL("image/png");
        resolve(dataURL);
      };

      img.onerror = () => {
        URL.revokeObjectURL(objectURL);
        reject(new Error("Failed to load image from buffer."));
      };

      img.src = objectURL;
    });
  }

  const detectSudoku = async (buf) => {
    if (!detectInference) {
      console.warn("Session not ready for inference.");
      return;
    }
    const [input, img_width, img_height, imgSrc] = await prepareInput(buf);
    setSudokuImage(imgSrc);
    const images = new ort.Tensor(Float32Array.from(input), [1, 3, 640, 640]);
    const outputs = await detectInference({ images: images });
    const output = outputs["output0"].data;
    const boxes = processOutput(output, img_width, img_height);
    if (boxes.length == 0) {
      setNoSudokuAlert(true);
      setProcessing(false);
      return;
    }
    const box = boxes[0];
    cropImage(buf, img_width, img_height, box).then(async (dataUrl) => {
      const response = await fetch(dataUrl);
      const blob = await response.blob();
      const file = new File([blob], "sudoku", { type: blob.type });
      setCropedImage(file);
    });
  };

  const ocrSudoku = async (buf) => {
    const [input, img_width, img_height] = await prepareInput(buf);
    const images = new ort.Tensor(Float32Array.from(input), [1, 3, 640, 640]);
    const outputs = await ocrInference({ images: images });
    const boxes = processOutput(
      outputs["output0"].data,
      img_width,
      img_height,
      "ocr",
    );
    const ocrCells = ocrPostProcess(img_width, img_height, boxes);
    const cells = initCellsFromOcr(ocrCells);
    setEditCells(cells);
    setProcessing(false);
    setEdit(true);
  };

  const handleUploadFile = async (files) => {
    if (files.length <= 0) {
      return;
    }
    const file = files[0];
    setProcessing(true);
    setInputFile(file);
  };
  const handleBack = () => {
    setEdit(false);
  };

  const handlePaste = async () => {
    const clipboardItems = await navigator.clipboard.read();
    if (clipboardItems.length === 0) {
      notifications.show({
        color: "red",
        title: "not found",
        message: "no image in clipboard",
      });
      return;
    }
    const clipboardItem = clipboardItems[0];
    const imageTypes = clipboardItem.types?.filter((type) =>
      type.startsWith("image/"),
    );
    if (imageTypes.length === 0) {
      notifications.show({
        color: "red",
        title: "not found",
        message: "no image in clipboard",
      });
      return;
    }
    const imageType = imageTypes[0];
    const blob = await clipboardItem.getType(imageType);
    setInputFile(blob);
    setProcessing(true);
  };

  const editor = (() => {
    return (
      <>
        <Button onClick={handleBack} variant="subtle">
          <IconChevronLeft />
        </Button>

        <Editor sudokuImage={sudokuImage} />
      </>
    );
  })();

  const uploader = (() => {
    return (
      <>
        <div className={styles.container}>
          <h1>Sudoku Scanner</h1>
          <Button onClick={handlePaste} size="lg" radius={5}>
            Paste from ClipBoard
          </Button>
          <Dropzone
            loading={processing}
            onDrop={handleUploadFile}
            maxSize={5 * 1024 ** 2}
            accept={IMAGE_MIME_TYPE}
          >
            <Group
              justify="center"
              gap="xl"
              mih={220}
              style={{ pointerEvents: "none" }}
            >
              <Dropzone.Accept>
                <IconUpload
                  size={52}
                  color="var(--mantine-color-blue-6)"
                  stroke={1.5}
                />
              </Dropzone.Accept>
              <Dropzone.Reject>
                <IconX
                  size={52}
                  color="var(--mantine-color-red-6)"
                  stroke={1.5}
                />
              </Dropzone.Reject>
              <Dropzone.Idle>
                <IconPhoto
                  size={52}
                  color="var(--mantine-color-dimmed)"
                  stroke={1.5}
                />
              </Dropzone.Idle>

              <div>
                <Text size="xl" inline>
                  Drag images here or click to select files
                </Text>
                <Text size="sm" c="dimmed" inline mt={7}>
                  each file should not exceed 5mb
                </Text>
              </div>
            </Group>
          </Dropzone>
          {noSudokuAlert && (
            <Alert onClose={() => setNoSudokuAlert(false)}>
              No Sudoku found
            </Alert>
          )}
        </div>
      </>
    );
  })();

  return <>{edit ? editor : uploader}</>;
}
