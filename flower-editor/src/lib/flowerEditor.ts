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
        const {
          resources,
          states,
          transitions,
          overlays,
          references,
          intermediates,
        } = flow;
        for (const state of states) {
          state;
        }
        ctx.fillStyle = "#f392a3";
        ctx.fillRect(50, 50, 100, 100);

        isRendering.current = false;
      }
    });
  };

  const saveFlow = useCallback(async () => {
    const flowPath = await window.electron.showSaveFlowDialog();
    if (!!flowPath && flow) {
      try {
        const flowStr = json2flow(JSON.stringify(flow));
        console.log({ flowStr });
        await window.electron.writeFile(flowPath, flowStr);
      } catch (e) {
        console.warn({ e });
      }
    }
  }, [flow]);

  const loadFlow = async () => {
    const loadFlowPath = await window.electron.showOpenFlowDialog();
    if (!!loadFlowPath) {
      setFlowPath(loadFlowPath);
      const flowStr = await window.electron.readFile(loadFlowPath);
      console.log({ flowStr });
      try {
        const flow = JSON.parse(flow2json(flowStr));
        console.log({ flow });
        setFlow(flow);
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
      if (flow) {
        draw(flow);
      }
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
  }, [flow, size]);

  return { flowPath, ref, saveFlow, loadFlow, handleKeyInput, createFlow };
}
