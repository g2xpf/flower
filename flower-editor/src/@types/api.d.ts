export type Resource = string;
export type State = string;
export type Intermediate = string;
export type Transition = {
  from: State;
  intermediate: Intermediate | null;
  to: State;
};
export type Reference = {
  state: State;
  resource: Resource;
};
export type Overlay = {
  back: State;
  front: State;
};
export type Flow = {
  resources: Resource[];
  states: State[];
  references: Reference[];
  transitions: Transition[];
  intermediates: Intermediate[];
  overlays: Overlay[];
};

export type APIKey = "electron";
export type API = {
  showOpenFlowDialog: () => Promise<string | undefined>;
  showSaveFlowDialog: () => Promise<string | undefined>;
  writeFile: (path: string, content: string) => Promise<void>;
  readFile: (path: string) => Promise<string>;
  createFile: (path: string) => Promise<void>;
};
