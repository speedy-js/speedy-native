import { Suite } from 'benchmark'
import chalk from 'chalk'

import { transformSync } from '@babel/core'
import { transform } from '../node/lib'
import { transformSync as swcTransformSync, parseSync } from '@swc/core'

const code = `
import React from "react";
import ReactDOM from "react-dom";
import { Input, AutoComplete, InputProps, Radio } from "antd";
import { Button as AntButton } from "antd";
import { List } from "antd";
import Child from "./component/Child";

const Item = List.Item;

class Page extends React.Component<InputProps,any> {
    render() {
        return (
            <div className={"test"}>
                <div>Page</div>
                <Input/>
                <AntButton>Button</AntButton>
                <Radio.Group />
                <Item />
            </div>
        );
    }
}

ReactDOM.render(<Page/>, document.getElementById("root"));
`

const suite = new Suite('transform import')
const wasmPlugin = require.resolve('@speedy-js/speedy-wasm');

suite
  .add('Babel', () => {
    transformSync(code, {
      parserOpts: {
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
      },
      plugins: [
        ['babel-plugin-import', {
          "libraryName": "antd",
          "style": true,
        }]
      ],
      generatorOpts: {
        sourceMaps: true,
        sourceFileName: 'test.ts'
      }
    })
  })
  .add('Rust', () => {
    transform.transformBabelImport(code, {
      reactRuntime: true,
      babelImport: [
        {
          fromSource: 'antd',
          replaceCss: {
            camel2DashComponentName: true,
            replaceExpr: name => `antd/es/${name}/style/index.css`,
            ignoreStyleComponent: undefined,
          },
          replaceJs: {
            camel2DashComponentName: true,
            replaceExpr: name => `antd/es/${name}/index.js`,
            ignoreEsComponent: undefined,
          },
        },
      ],
    }).code;
  })
  .add('Wasm', () => {
    swcTransformSync(code, {
      jsc: {
        parser: {
          syntax: 'typescript',
          tsx: true,
        },
        target: "es2020",
        experimental: {
          plugins: [
            [wasmPlugin, {
              reactRuntime: true,
              babelImport: [
                {
                  fromSource: "antd",
                  replaceCss: {
                    replaceExpr: `antd/es/{}/style/index.css`,
                    ignoreStyleComponent: undefined,
                    camel2DashComponentName: true,
                  },
                  replaceJs: {
                    replaceExpr: `antd/es/{}/index.js`,
                    ignoreEsComponent: undefined,
                    camel2DashComponentName: true,
                  },
                },
              ],
            }]
          ],
        },
      },
    });
  })
  .on('cycle', function (event: Event) {
    console.info(String(event.target))
  })
  .on('complete', function (this: any) {
    console.info(
      `${this.name} bench suite: Fastest is ${chalk.green(
        this.filter('fastest').map('name')
      )}`
    )
  })
  .run()
