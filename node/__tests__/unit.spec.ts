import assert from 'assert';

import {parse} from '@babel/parser';
import traverse from '@babel/traverse';
import generate from '@babel/generator';
import {Program} from '@babel/types';
import {transform} from '../lib';
import { SourceMapConsumer } from 'source-map';
import * as process from "process";

describe('speedy_napi_cases', function speedyTest() {
    it('babel_import_transform should track type correctly', async () => {
        const code = `
import { InputProps, Button } from "antd";

{
    let InputProps = 1;
    console.log(InputProps);
}

function App(props: InputProps) {}
`;

        let target_code = `
import { InputProps } from "antd";
{
    let InputProps = 1;
    console.log(InputProps);
}

function App(props: InputProps) {}
`;
        const napi_res = transform.transformBabelImport(code, {
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
        });

        assert.equal(
            target_code.replace(/\ +/g, '').replace(/[\r\n]/g, ''),
            napi_res.code.replace(/\ +/g, '').replace(/[\r\n]/g, '')
        );
    });

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

class Page extends React.Component<any,any>{
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
            reactRuntime: true,
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

class Page extends React.Component<any,any>{
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
            reactRuntime: true,
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
import { Input, AutoComplete, InputProps, Radio } from "antd";
import Child from "./component/Child";

type Props = InputProps;

class Page extends React.Component<Props,any> {
    render() {
        return (
            <div className={"test"}>
                <div>Page</div>
                <Input/>
                <Radio.Group />
            </div>
        );
    }
}

ReactDOM.render(<Page/>, document.getElementById("root"));
`;

        let target_code = `
import "antd/es/radio/style/index.css";
import "antd/es/input/style/index.css";
import { Radio } from "antd/es/radio/index.js";
import { Input } from "antd/es/input/index.js";
import React from "react";
import ReactDOM from "react-dom";
import { InputProps } from "antd";
import Child from "./component/Child";

type Props = InputProps;

class Page extends React.Component<Props,any>{
    render() {
        return (
            <div className={"test"}>
                <div>Page</div>
                <Input/>
                <Radio.Group />
            </div>
        );
    }
}

ReactDOM.render(<Page / >, document.getElementById("root"));
        `;
        console.time('babel_import_swc_transfrom');
        process.env["rsdebug"] = "info";
        const napi_res = transform.transformBabelImport(code, {
            reactRuntime: true,
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

    it('babel_import_transfrom should track components ref correctly', async () => {
        const code = `
import React from "react";
import ReactDOM from "react-dom";
import { Radio, List } from "antd";

const Item = List.Item; // ref List

class Page extends React.Component<InputProps,any> {
    render() {
        return (
            <div className={"test"}>
                {/* ref Radio */}
                <Radio.RadioGroup.RadioItem />
                <Item />
            </div>
        );
    }
}

ReactDOM.render(<Page/>, document.getElementById("root"));
`;

        let target_code = `
import "antd/es/list/style/index.css";
import "antd/es/radio/style/index.css";
import { List } from "antd/es/list/index.js";
import { Radio } from "antd/es/radio/index.js";
import React from "react";
import ReactDOM from "react-dom";

const Item = List.Item;

class Page extends React.Component<InputProps,any>{
    render() {
        return (
            <div className={"test"}>
                {}
                <Radio.RadioGroup.RadioItem />
                <Item />
            </div>
        );
    }
}

ReactDOM.render(<Page / >, document.getElementById("root"));
        `;
        console.time('babel_import_swc_transfrom');
        process.env["rsdebug"] = "info";
        const napi_res = transform.transformBabelImport(code, {
            reactRuntime: true,
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

    it('remove_call_transform should work with simple case', async () => {
        let code = `
import React from 'react';
import ReactDOM from "react-dom";
import { useEffect } from 'react';

function App() {
    const [num, setNum] = React.useState(1);
    React.useState(2);
    
    React.useEffect(() => {
        setNum(2);
    }, []);

    useEffect(() => {
        setNum(3);
    }, []);

    return <div>{num}</div>;
}
ReactDOM.render(<Page/>, document.getElementById("root"));
`;

        let target_code = `
import React from "react";
import ReactDOM from "react-dom";
import { useEffect } from "react";

function App() {
    const [num, setNum] = React.useState(1);
    React.useState(2);

    return <div>{num}</div>;
}

ReactDOM.render(<Page/>, document.getElementById("root"));
`;

        const napi_res = transform.transformBabelImport(code, {
            removeUseEffect: true,
        })

        assert.equal(
            target_code.replace(/\ +/g, '').replace(/[\r\n]/g, ''),
            napi_res.code.replace(/\ +/g, '').replace(/[\r\n]/g, '')
        );
    })

    it('remove_call_transform should work with complex case', async () => {
        let code = `
import Recta from 'react';
import ReactDOM from "react-dom";
import { useEffect as effectUse } from 'react';

function useEffect() {
    console.log("not delete");
}

{
    useEffect();
}

function App() {
    const [num, setNum] = Recta.useState(1);
    Recta.useState(1);
    
    Recta.useEffect(() => {
        setNum(2);
    }, []);

    effectUse(() => {
        setNum(3);
    }, []);

    {
        effectUse(() => {
            setNum(4);
        }, []);
    }

    {
        const useEffect = () => 2;
        const effectUse = () => 1;
        useEffect();
        effectUse();
    }

    return <div>{num}</div>;
}
ReactDOM.render(<Page/>, document.getElementById("root"));
`;

        let target_code = `
import Recta from "react";
import ReactDOM from "react-dom";
import { useEffect as effectUse } from "react";

function useEffect() {
    console.log("not delete");
}

{
    useEffect();
}

function App() {
    const [num, setNum] = Recta.useState(1);
    Recta.useState(1);

    {}

    {
        const useEffect = () => 2;
        const effectUse = () => 1;
        useEffect();
        effectUse();
    }

    return <div>{num}</div>;
}

ReactDOM.render(<Page/>, document.getElementById("root"));
`;

        const napi_res = transform.transformBabelImport(code, {
            removeUseEffect: true,
        })

        assert.equal(
            target_code.replace(/\ +/g, '').replace(/[\r\n]/g, ''),
            napi_res.code.replace(/\ +/g, '').replace(/[\r\n]/g, '')
        );
    })
    
    it(`remove_call_transform should work correctly among scope`, async () => {
        // https://github.com/speedy-js/speedy-native/pull/27#issuecomment-1195278186
        let code = `
import { useEffect } from 'react';

{
    const useEffect = () => {}
    useEffect()
}

function App() {
    useEffect()
}`;

        let target_code = `
import { useEffect } from "react";
{
    const useEffect = ()=>{};
    useEffect();
}
function App() {}
`;

        const napi_res = transform.transformBabelImport(code, {
            removeUseEffect: true,
        })

        assert.equal(
            target_code.replace(/\ +/g, '').replace(/[\r\n]/g, ''),
            napi_res.code.replace(/\ +/g, '').replace(/[\r\n]/g, '')
        );
    })

    it(`remove_call source map test`, async () => {
        let code = `
import React from "react";
import ReactDOM from "react-dom";
import { useEffect } from 'react';

function App() {
    const [num, setNum] = React.useState(1);
    
    React.useEffect(() => {
        setNum(2);
    }, []);

    useEffect(() => {
        setNum(3);
    }, []);

    return (
        <div>{num}</div>
    );
}
ReactDOM.render(<Page/>, document.getElementById("root"));
`;

        let target_code = `import React from "react";
import ReactDOM from "react-dom";
import { useEffect } from "react";
function App() {
    const [num, setNum] = React.useState(1);
    return (<div >{num}</div>);
}
ReactDOM.render(<Page />, document.getElementById("root"));
`;

        const napi_res = transform.transformBabelImport(code, {
            removeUseEffect: true
        });

        const consumer = await new SourceMapConsumer(napi_res.map as any);

        const position1 = consumer.originalPositionFor({
            line: 5,
            column: 4
        })
        const position2 = consumer.originalPositionFor({
            line: 6,
            column: 12
        })
        const position3 = consumer.originalPositionFor({
            line: 8,
            column: 1
        })

        assert.equal(napi_res.code, target_code);
        assert.equal(position1.line, 7);
        assert.equal(position1.column, 4);
        assert.equal(position2.line, 17);
        assert.equal(position2.column, 4);
        assert.equal(position3.line, 21);
        assert.equal(position3.column, 0);
    })
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
