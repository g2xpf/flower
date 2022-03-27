import {
  RefObject,
  useEffect,
  useState,
  useRef,
  useCallback,
  KeyboardEventHandler,
  KeyboardEvent,
} from "react";
import { Flow } from "../@types/api";
import { json2flow, flow2json } from "../pkg";

export type FlowerEditor = {
  saveFlow: () => Promise<void>;
  loadFlow: () => Promise<void>;
  createFlow: () => Promise<void>;
  flowPath: string | null;
  handleKeyInput: KeyboardEventHandler<HTMLCanvasElement>;
  ref: RefObject<HTMLCanvasElement>;
};

export interface Size {
  width: number | undefined;
  height: number | undefined;
}

const fillObject = <O extends {}>(
  ctx: CanvasRenderingContext2D,
  obj: O,
  x: number,
  y: number
) => {
  let linesStr: string;

  const spacePerTab = 4;
  const spaces = " ".repeat(spacePerTab);
  const measure = ctx.measureText("");
  const fontHeight =
    measure.fontBoundingBoxAscent + measure.fontBoundingBoxDescent;

  if (typeof obj === "string") {
    linesStr = obj;
  } else {
    const s = JSON.stringify(obj, null, "\t");
    linesStr = s;
  }

  const lines = linesStr.split("\n");
  for (let i = 0, len = lines.length; i < len; ++i) {
    let line = lines[i];
    let numCount = 0;
    while (line.startsWith("\t")) {
      line = line.slice(1);
      numCount++;
    }
    const tabSpace = spaces.repeat(numCount);
    const height = y + i * fontHeight;

    ctx.fillText(`${tabSpace}${line}`, x, height);
  }
};

export function useFlowerEditor(size: Size): FlowerEditor {
  const [flowPath, setFlowPath] = useState<string | null>(null);

  const canvasRef = useRef<HTMLCanvasElement>(null);
  const contextRef = useRef<CanvasRenderingContext2D | null>(null);
  const flowRef = useRef<Flow>();
  const rawFlowStrRef = useRef<string>();
  const frameRequestHandleRef = useRef<number>();

  // draw flow
  const draw = (timer: DOMHighResTimeStamp) => {
    if (contextRef.current && flowRef.current) {
      const ctx = contextRef.current;
      const flow = Object.assign({}, flowRef.current);

      const { width, height } = ctx.canvas;
      ctx.clearRect(0, 0, width, height);
      fillObject(ctx, flow, 0, 10);
      rawFlowStrRef.current && fillObject(ctx, rawFlowStrRef.current, 200, 10);
    }

    frameRequestHandleRef.current = requestAnimationFrame(draw);
  };

  useEffect(() => {
    frameRequestHandleRef.current = requestAnimationFrame(draw);

    return () => {
      if (frameRequestHandleRef.current) {
        cancelAnimationFrame(frameRequestHandleRef.current);
      }
    };
  }, []);

  const saveFlow = async () => {
    const flowPath = await window.electron.showSaveFlowDialog();
    if (!!flowPath && flowRef.current) {
      try {
        const flowStr = json2flow(JSON.stringify(flowRef.current));
        console.log({ flowStr });
        await window.electron.writeFile(flowPath, flowStr);
      } catch (e) {
        console.warn({ e });
      }
    }
  };

  const loadFlow = async () => {
    const loadFlowPath = await window.electron.showOpenFlowDialog();
    if (!!loadFlowPath) {
      setFlowPath(loadFlowPath);
      const flowStr = await window.electron.readFile(loadFlowPath);
      console.log({ flowStr });
      try {
        const flow = JSON.parse(flow2json(flowStr));
        console.log({ flow });
        flowRef.current = flow;
        rawFlowStrRef.current = flowStr;

        draw(flow);
      } catch (e) {
        console.warn(e);
      }
    }
  };
  const createFlow = async () => {
    const createFlowPath = await window.electron.showSaveFlowDialog();
    if (!!createFlowPath) {
      await window.electron.createFile(createFlowPath);
      setFlowPath(createFlowPath);
    }
  };
  const handleKeyInput = (e: KeyboardEvent<HTMLCanvasElement>) => {};

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) {
      return;
    }
    const ctx = canvas.getContext("2d");
    if (ctx && size.width && size.height) {
      const dpr = window.devicePixelRatio || 1;
      const width = Math.floor(size.width);
      const height = Math.floor(size.height);

      ctx.fillStyle = "#f392a3";
      canvas.style.width = `${width}px`;
      canvas.style.height = `${height}px`;
      canvas.width = width * dpr;
      canvas.height = height * dpr;
      ctx.scale(dpr, dpr);
    }
    contextRef.current = ctx;
  }, [size]);

  return {
    flowPath,
    ref: canvasRef,
    saveFlow,
    loadFlow,
    handleKeyInput,
    createFlow,
  };
}
