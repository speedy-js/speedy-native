import {transformBabelImport, sassRender} from '../index';

declare const transform: {
    transformBabelImport: typeof transformBabelImport;
    sassRender: typeof sassRender;
};

export {transform};
