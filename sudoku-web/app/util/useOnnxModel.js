import React, { useState, useEffect, useRef, useCallback } from "react";
import * as ort from "onnxruntime-web";

const useOnnxModel = (modelPath, sessionOptions = {}) => {
  const [session, setSession] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const sessionRef = useRef(null);

  useEffect(() => {
    let isMounted = true;

    const loadModel = async () => {
      setLoading(true);
      setError(null);
      try {
        if (!modelPath) {
          throw new Error("modelPath cannot be empty.");
        }
        const newSession = await ort.InferenceSession.create(
          modelPath,
          sessionOptions,
        );
        if (isMounted) {
          setSession(newSession);
          sessionRef.current = newSession;
          setLoading(false);
        }
      } catch (err) {
        console.error("Failed to load ONNX model:", err);
        if (isMounted) {
          setError(err);
          setLoading(false);
        }
      }
    };

    loadModel();

    return () => {
      isMounted = false;
      if (sessionRef.current) {
        sessionRef.current = null;
      }
    };
  }, [modelPath, JSON.stringify(sessionOptions)]);

  const runInference = useCallback(async (inputs, runOptions = {}) => {
    if (!sessionRef.current) {
      console.warn("Attempted to run inference before ONNX model was loaded.");
      return null;
    }
    try {
      const outputs = await sessionRef.current.run(inputs, runOptions);
      return outputs;
    } catch (err) {
      console.error("Error during ONNX model inference:", err);
      throw err;
    }
  }, []);

  return { session, loading, error, runInference };
};

export default useOnnxModel;
