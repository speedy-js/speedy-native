/* eslint-disable */

export class ExternalObject<T> {
  readonly "": {
    readonly "": unique symbol;
    [K: symbol]: T;
  };
}
export interface TransformOutput {
  code: string;
  map?: string | undefined | null;
}
export interface TransformConfig {
  reatRuntime?: boolean | undefined | null;
  babelImport?: Array<BabelImportConfig> | undefined | null;
}
export interface BabelImportConfig {
  fromSource: string;
  replaceCss?: RepalceCssConfig | undefined | null;
  replaceJs?: RepalceSpecConfig | undefined | null;
}
export interface RepalceSpecConfig {
  replaceExpr: string;
  ignoreEsComponent?: Array<string> | undefined | null;
  lower?: boolean | undefined | null;
}
export interface RepalceCssConfig {
  ignoreStyleComponent?: Array<string> | undefined | null;
  replaceExpr: string;
  lower?: boolean | undefined | null;
}
export function transformBabelImport(
  code: string,
  config: TransformConfig,
  filename?: string | undefined | null,
  target?: string | undefined | null
): TransformOutput;
