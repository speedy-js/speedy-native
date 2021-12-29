import assert from 'assert';

import {parse} from '@babel/parser';
import traverse from '@babel/traverse';
import generate from '@babel/generator';
import {Program} from '@babel/types';
import {execSync} from 'child_process';
import {transform} from '../lib';
import * as process from "process";

describe('speedy_napi_cases', function speedyTest() {
    it('babel_import_transfrom', async () => {
        const code = `
import React from "react";
import ReactDOM from "react-dom";
import { Button, Input } from "antd";
import Child from "./component/Child";

class Page extends React.Component<any,any> {
    render() {
        return (
            <div className={"test"}>
                <div>Page</div>
                <Child/>
                <Button>click me</Button>
                <Input/>
            </div>
        );
    }
}

ReactDOM.render(<Page/>, document.getElementById("root"));
`;

        let target_code = `
import "antd/es/input/style/index.css";
import "antd/es/button/style/index.css";
import Input from "antd/es/input/index.js";
import Button from "antd/es/button/index.js";
import React from "react";
import ReactDOM from "react-dom";
import Child from "./component/Child";

class Page extends React.Component{
    render() {
        return (
            <div className={"test"}>
                <div>Page</div>
                <Child/>
                <Button>click me</Button>
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
                        replaceExpr: 'antd/es/{}/style/index.css',
                        lower: true,
                        ignoreStyleComponent: undefined,
                    },
                    replaceJs: {
                        replaceExpr: 'antd/es/{}/index.js',
                        lower: true,
                        ignoreEsComponent: undefined,
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
