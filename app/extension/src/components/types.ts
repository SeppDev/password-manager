export type FunctionEvent = (event?: Event) => Promise<void> | void;
export type Inputs = { [key: string]: string };
export type PromptData = {
  edit?: boolean;
  url: string;
  inputs: Inputs;
};
