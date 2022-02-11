export interface PluginHint<Name extends string> {
  name: Name,
}

export interface TransfromOutput {
  code: string,
  map?: string,
}