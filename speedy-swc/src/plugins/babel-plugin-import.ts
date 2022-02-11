import { PluginHint } from "../types";

export interface BabelPluginImportPlugin extends PluginHint<'babel-plugin-import'> {
  libraryName: string,
  style: boolean,   // or 'css'
}