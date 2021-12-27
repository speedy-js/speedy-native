import { transformBabelImport } from '../index';

declare const transform: {
  transformBabelImport: typeof transformBabelImport;
};

export { transform };
