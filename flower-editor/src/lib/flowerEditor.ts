import {
  RefObject,
  useEffect,
  useState,
  useRef,
  useCallback,
  KeyboardEventHandler,
  KeyboardEvent,
} from "react";

export type FlowerEditor = {
  saveFlow: () => Promise<void>;
  loadFlow: () => Promise<void>;
  createFlow: () => Promise<void>;
  flowPath: string | null;
  handleKeyInput: KeyboardEventHandler<HTMLCanvasElement>;
  ref: RefObject<HTMLCanvasElement>;
};

export type Flow = {};
export interface Size {
  width: number | undefined;
  height: number | undefined;
}

export function useFlowerEditor(size: Size): FlowerEditor {
  const ref = useRef<HTMLCanvasElement>(null);
  const context = useRef<CanvasRenderingContext2D | null>(null);
  const [flow, setFlow] = useState<Flow | null>(null);
  const [flowPath, setFlowPath] = useState<string | null>(null);

  const draw = (flow: Flow) => {
    if (isRendering.current) {
      console.log("skipped");
      return;
    }
    console.log(flow);
    isRendering.current = true;
    requestAnimationFrame((_time: DOMHighResTimeStamp) => {
      const ctx = context.current;
      if (ctx) {
        ctx.fillStyle = "#f392a3";
        ctx.fillRect(50, 50, 100, 100);

        isRendering.current = false;
      }
    });
  };

  const saveFlow = useCallback(async () => {
    const flowPath = await window.electron.showSaveFlowDialog();
    if (!!flowPath && flow) {
      await window.electron.saveFlow(flowPath, flow);
    }
  }, [flow]);

  const loadFlow = async () => {
    const loadFlowPath = await window.electron.showOpenFlowDialog();
    if (!!loadFlowPath) {
      setFlowPath(loadFlowPath);
      const flow = await window.electron.openFlow(loadFlowPath);
      console.log({ flow });
      setFlow(flow);
      draw(flow);
    }
  };
  const createFlow = async () => {
    const createFlowPath = await window.electron.showSaveFlowDialog();
    if (!!createFlowPath) {
      await window.electron.createFlow(createFlowPath);
      setFlowPath(createFlowPath);
      draw({});
    }
  };
  const handleKeyInput = (e: KeyboardEvent<HTMLCanvasElement>) => {};

  const isRendering = useRef(false);

  useEffect(() => {
    const canvas = ref.current;
    if (!canvas) {
      return;
    }
    const ctx = canvas.getContext("2d");
    if (ctx && size.width && size.height) {
      ctx.canvas.width = size.width;
      ctx.canvas.height = size.height;
    }
    context.current = ctx;

    if (flow) {
      draw(flow);
    }
  }, [size]);

  return { flowPath, ref, saveFlow, loadFlow, handleKeyInput, createFlow };
}
