"use client";
import { useCallback, useEffect, useMemo, useState } from "react";
import { IconChevronLeft } from "@tabler/icons-react";

import { Group, Text, Button, Alert } from "@mantine/core";
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

export default function Scanner() {
  const [edit, setEdit] = useState(false);
  const [editCells, setEditCells] = useAtom(editCellsAtom);
  const [sudokuImage, setSudokuImage] = useState("");
  const [processing, setProcessing] = useState(false);
  const [noSudokuAlert, setNoSudokuAlert] = useState(false);
  const [inputFile, setInputFile] = useState(null);

  const sessionOptions = useMemo(() => ({
    executionProviders: ["webgl", "wasm"],
    graphOptimizationLevel: "all",
  }), []);
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

  const scanSudoku = useCallback(async (buf) => {
    if (!detectInference || !ocrInference) return;
    setProcessing(true);
    setNoSudokuAlert(false);
    try {
    const [input, img_width, img_height, imgSrc] = await prepareInput(buf);
    setSudokuImage(imgSrc);
    const images = new ort.Tensor(Float32Array.from(input), [1, 3, 640, 640]);
    const outputs = await detectInference({ images: images });
    const output = outputs["output0"].data;
    const boxes = processOutput(output, img_width, img_height);
    if (boxes.length == 0) {
      setNoSudokuAlert(true);
      return;
    }
    const dataUrl = await cropImage(buf, img_width, img_height, boxes[0]);
    const blob = await (await fetch(dataUrl)).blob();
    const [ocrInput, cropWidth, cropHeight] = await prepareInput(blob);
    const ocrImages = new ort.Tensor(Float32Array.from(ocrInput), [1, 3, 640, 640]);
    const ocrOutputs = await ocrInference({ images: ocrImages });
    const ocrBoxes = processOutput(
      ocrOutputs["output0"].data,
      cropWidth,
      cropHeight,
      "ocr",
    );
    const ocrCells = ocrPostProcess(cropWidth, cropHeight, ocrBoxes);
    const cells = initCellsFromOcr(ocrCells);
    setEditCells(cells);
    setEdit(true);
    } catch (error) {
      console.error("Sudoku scan failed:", error);
      notifications.show({
        color: "red",
        title: "Sudoku scan failed",
        message: "Could not read that image. Try a clearer screenshot or photo.",
      });
    } finally {
      setProcessing(false);
      setInputFile(null);
    }
  }, [detectInference, ocrInference, setEditCells]);

  useEffect(() => {
    if (inputFile && detectSession && ocrSession && !detectLoading && !ocrLoading) {
      scanSudoku(inputFile);
    }
  }, [detectLoading, detectSession, inputFile, ocrLoading, ocrSession, scanSudoku]);

  useEffect(() => {
    if (!detectError && !ocrError) return;
    setProcessing(false);
    notifications.show({
      color: "red",
      title: "Scanner is unavailable",
      message: "The OCR model could not be loaded. Refresh the page and try again.",
    });
  }, [detectError, ocrError]);

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
    try {
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
    } catch (error) {
      console.error("Clipboard read failed:", error);
      notifications.show({
        color: "red",
        title: "Clipboard unavailable",
        message: "Allow clipboard access or upload an image instead.",
      });
    }
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
        <div className="mx-auto flex w-full max-w-3xl flex-col gap-3">
          <h1>Sudoku Scanner</h1>
          <Button onClick={handlePaste} size="lg" radius={5}>
            Paste from clipboard
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
                  Each file should not exceed 5 MB
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
