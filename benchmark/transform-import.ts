import { Suite } from 'benchmark'
import chalk from 'chalk'

import { parse } from '@babel/parser'
import traverse from '@babel/traverse'
import generate from '@babel/generator'
import { Program } from '@babel/types'

import { transform } from '../node/lib'

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
`

function babelImport(code: string, lib: string, expr: string) {
  const ast = parse(code, {
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
  })

  let pro: Program
  traverse(ast, {
    enter(path) {
      if (path.isProgram()) {
        pro = path.node
      }
      const isMatch = (source: string) => {
        return lib == source
      }
      if (path.isImportDeclaration() && isMatch(path.node.source.value)) {
        const origin_names = path.node.specifiers.map((p) => {
          return expr.replace('{}', p.local.name)
        })
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
          } as any)
        })
      }
    },
  })

  const res = generate(
    ast,
    { sourceMaps: true, sourceFileName: 'test.js' },
    code
  )
  return res
}

const suite = new Suite('transform import')

suite
  .add('Babel', () => {
    babelImport(code, 'antd', `antd/es/{}/style/index.css`)
  })
  .add('Rust', () => {
    transform.transformBabelImport(code, {
      reactRuntime: true,
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
      ],
    })
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
