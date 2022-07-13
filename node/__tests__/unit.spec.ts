import assert from 'assert';

import {parse} from '@babel/parser';
import traverse from '@babel/traverse';
import generate from '@babel/generator';
import {Program} from '@babel/types';
import {transform} from '../lib';
import * as process from "process";

describe('speedy_napi_cases', function speedyTest() {
    it('babel_import_transfrom with camel2DashComponentName true', async () => {
        const code = `
import React from "react";
import ReactDOM from "react-dom";
import { Input, AutoComplete } from "antd";
import Child from "./component/Child";
import { Button as AntButton } from "antd";

class Page extends React.Component<any,any> {
    render() {
        return (
            <div className={"test"}>
                <div>Page</div>
                <Child/>
                <AntButton>click me</AntButton>
                <Input/>
                <AutoComplete />
            </div>
        );
    }
}

ReactDOM.render(<Page/>, document.getElementById("root"));
`;

        let target_code = `
import "antd/es/button/style/index.css";
import "antd/es/auto-complete/style/index.css";
import "antd/es/input/style/index.css";
import AntButton from "antd/es/button/index.js";
import AutoComplete from "antd/es/auto-complete/index.js";
import Input from "antd/es/input/index.js";
import React from "react";
import ReactDOM from "react-dom";
import Child from "./component/Child";

class Page extends React.Component{
    render() {
        return (
            <div className={"test"}>
                <div>Page</div>
                <Child/>
                <AntButton>click me</AntButton>
                <Input/>
                <AutoComplete />
            </div>
        );
    }
}

ReactDOM.render(<Page / >, document.getElementById("root"));
        `;
        console.time('babel_import_swc_transfrom');
        process.env["rsdebug"] = "info";
        const napi_res = transform.transformBabelImport(code, {
            reatRuntime: true,
            babelImport: [
                {
                    fromSource: 'antd',
                    replaceCss: {
                        replaceExpr: (ident: string) => {
                            return `antd/es/${ident}/style/index.css`;
                        },
                        lower: true,
                        ignoreStyleComponent: undefined,
                        camel2DashComponentName: true
                    },
                    replaceJs: {
                        replaceExpr: (ident: string) => {
                            return `antd/es/${ident}/index.js`;
                        },
                        lower: true,
                        ignoreEsComponent: undefined,
                        camel2DashComponentName: true
                    },
                },
            ]
        })
        console.timeEnd('babel_import_swc_transfrom');

        // 执行同样的 babel 操作
        console.time('babel_import_babeljs_transfrom');

        const babel_res = babel_impl_bableimport(code, 'antd', `antd/es/{}/style/index.css`);
        console.timeEnd('babel_import_babeljs_transfrom');

        assert.equal(
            target_code.replace(/\ +/g, '').replace(/[\r\n]/g, ''),
            napi_res.code.replace(/\ +/g, '').replace(/[\r\n]/g, '')
        );
    });

    it('babel_import_transfrom with transformToDefaultImport set false', async () => {
        const code = `
import React from "react";
import ReactDOM from "react-dom";
import { Input, AutoComplete } from "antd";
import Child from "./component/Child";
import { Button as AntButton } from "antd";

class Page extends React.Component<any,any> {
    render() {
        return (
            <div className={"test"}>
                <div>Page</div>
                <Child/>
                <AntButton>click me</AntButton>
                <Input/>
                <AutoComplete />
            </div>
        );
    }
}

ReactDOM.render(<Page/>, document.getElementById("root"));
`;

        let target_code = `
import "antd/es/button/style/index.css";
import "antd/es/auto-complete/style/index.css";
import "antd/es/input/style/index.css";
import { Button as AntButton } from "antd/es/button/index.js";
import { AutoComplete } from "antd/es/auto-complete/index.js";
import { Input } from "antd/es/input/index.js";
import React from "react";
import ReactDOM from "react-dom";
import Child from "./component/Child";

class Page extends React.Component{
    render() {
        return (
            <div className={"test"}>
                <div>Page</div>
                <Child/>
                <AntButton>click me</AntButton>
                <Input/>
                <AutoComplete />
            </div>
        );
    }
}

ReactDOM.render(<Page / >, document.getElementById("root"));
        `;
        console.time('babel_import_swc_transfrom');
        process.env["rsdebug"] = "info";
        const napi_res = transform.transformBabelImport(code, {
            reatRuntime: true,
            babelImport: [
                {
                    fromSource: 'antd',
                    replaceCss: {
                        replaceExpr: (ident: string) => {
                            return `antd/es/${ident}/style/index.css`;
                        },
                        lower: true,
                        ignoreStyleComponent: undefined,
                        camel2DashComponentName: true,
                    },
                    replaceJs: {
                        replaceExpr: (ident: string) => {
                            return `antd/es/${ident}/index.js`;
                        },
                        lower: true,
                        ignoreEsComponent: undefined,
                        transformToDefaultImport: false,
                        camel2DashComponentName: true,
                    },
                },
            ]
        })
        console.timeEnd('babel_import_swc_transfrom');

        // 执行同样的 babel 操作
        console.time('babel_import_babeljs_transfrom');

        const babel_res = babel_impl_bableimport(code, 'antd', `antd/es/{}/style/index.css`);
        console.timeEnd('babel_import_babeljs_transfrom');

        assert.equal(
            target_code.replace(/\ +/g, '').replace(/[\r\n]/g, ''),
            napi_res.code.replace(/\ +/g, '').replace(/[\r\n]/g, '')
        );
    });

    it('babel_import_transfrom should tree shaking (ts_type and unused components)', async () => {
        const code = `
import React from "react";
import ReactDOM from "react-dom";
import { Input, AutoComplete, InputProps } from "antd";
import Child from "./component/Child";

class Page extends React.Component<InputProps,any> {
    render() {
        return (
            <div className={"test"}>
                <div>Page</div>
                <Input/>
            </div>
        );
    }
}

ReactDOM.render(<Page/>, document.getElementById("root"));
`;

        let target_code = `
import "antd/es/input/style/index.css";
import { Input } from "antd/es/input/index.js";
import React from "react";
import ReactDOM from "react-dom";
import Child from "./component/Child";

class Page extends React.Component{
    render() {
        return (
            <div className={"test"}>
                <div>Page</div>
                <Input/>
            </div>
        );
    }
}

ReactDOM.render(<Page / >, document.getElementById("root"));
        `;
        console.time('babel_import_swc_transfrom');
        process.env["rsdebug"] = "info";
        const napi_res = transform.transformBabelImport(code, {
            reatRuntime: true,
            babelImport: [
                {
                    fromSource: 'antd',
                    replaceCss: {
                        replaceExpr: (ident: string) => {
                            return `antd/es/${ident}/style/index.css`;
                        },
                        lower: true,
                        ignoreStyleComponent: undefined,
                        camel2DashComponentName: true,
                    },
                    replaceJs: {
                        replaceExpr: (ident: string) => {
                            return `antd/es/${ident}/index.js`;
                        },
                        lower: true,
                        ignoreEsComponent: undefined,
                        transformToDefaultImport: false,
                        camel2DashComponentName: true,
                    },
                },
            ]
        })
        console.timeEnd('babel_import_swc_transfrom');

        // 执行同样的 babel 操作
        console.time('babel_import_babeljs_transfrom');

        const babel_res = babel_impl_bableimport(code, 'antd', `antd/es/{}/style/index.css`);
        console.timeEnd('babel_import_babeljs_transfrom');

        assert.equal(
            target_code.replace(/\ +/g, '').replace(/[\r\n]/g, ''),
            napi_res.code.replace(/\ +/g, '').replace(/[\r\n]/g, '')
        );
    });
});

/*
 * babel 同样实现 性能比对函数
 */
function babel_impl_bableimport(code: string, lib: string, expr: string) {
    /** 解析源码AST树 */
    const ast: any = parse(code, {
        sourceType: 'module',
        sourceFilename: undefined,
        plugins: [
            'typescript',
            'jsx',
            'decorators-legacy',
            'classProperties',
            'bigInt',
            'importMeta',
            'optionalChaining',
            'nullishCoalescingOperator',
            'importMeta',
            'optionalCatchBinding',
        ],
    });

    const change_ast = (ast: any) => {
        let pro: Program;
        traverse(ast, {
            enter(path) {
                if (path.isProgram()) {
                    pro = path.node;
                }
                const matchsource = (source: string) => {
                    return lib == source;
                };
                if (path.isImportDeclaration() && matchsource(path.node.source.value)) {
                    const origin_names = path.node.specifiers.map((p) => {
                        return expr.replace('{}', p.local.name);
                    });
                    origin_names.forEach((style_source) => {
                        pro.body.unshift({
                            type: 'ImportDeclaration',
                            specifiers: [],
                            source: {
                                type: 'StringLiteral',
                                extra: {
                                    rawValue: style_source,
                                    raw: `'${style_source}'`,
                                },
                                value: style_source,
                            },
                        } as any);
                    });
                }
                return;
            },
        });
    };

    change_ast(ast);
    const res = generate(ast, {sourceMaps: true, sourceFileName: 'test.js'}, code);
    return res;
}
