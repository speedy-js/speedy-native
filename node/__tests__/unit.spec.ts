import { transform } from "../lib";
import { SourceMapConsumer } from "source-map";

describe("speedy-napi: babel-import", () => {
  it("babel_import_transform should track type correctly", async () => {
    const code = `
import { InputProps, Button } from "antd";

{
    let InputProps = 1;
    console.log(InputProps);
}

function App(props: InputProps) {}
`;
    const napi_res = transform.transformBabelImport(code, {
      babelImport: [
        {
          fromSource: "antd",
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
            camel2DashComponentName: true,
          },
        },
      ],
    });

    expect(napi_res.code).toMatchSnapshot();
  });

  it("babel_import_transfrom with camel2DashComponentName true", async () => {
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

    const napi_res = transform.transformBabelImport(code, {
      reactRuntime: true,
      babelImport: [
        {
          fromSource: "antd",
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
            camel2DashComponentName: true,
          },
        },
      ],
    });

    expect(napi_res.code).toMatchSnapshot();
  });

  it("babel_import_transfrom with transformToDefaultImport set false", async () => {
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

    const napi_res = transform.transformBabelImport(code, {
      reactRuntime: true,
      babelImport: [
        {
          fromSource: "antd",
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
      ],
    });

    expect(napi_res.code).toMatchSnapshot();
  });

  it("babel_import_transfrom should tree shaking (ts_type and unused components)", async () => {
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

    const napi_res = transform.transformBabelImport(code, {
      reactRuntime: true,
      babelImport: [
        {
          fromSource: "antd",
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
      ],
    });

    expect(napi_res.code).toMatchSnapshot();
  });

  it("babel_import_transfrom should track components ref correctly", async () => {
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

    const napi_res = transform.transformBabelImport(code, {
      reactRuntime: true,
      babelImport: [
        {
          fromSource: "antd",
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
      ],
    });

    expect(napi_res.code).toMatchSnapshot();
  });
});

describe("speedy-napi: remove call", () => {
  it("remove_call_transform should work with simple case", async () => {
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

    const napi_res = transform.transformBabelImport(code, {
      removeUseEffect: true,
    });

    expect(napi_res.code).toMatchSnapshot();
  });

  it("remove_call_transform should work with complex case", async () => {
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

    const napi_res = transform.transformBabelImport(code, {
      removeUseEffect: true,
    });

    expect(napi_res.code).toMatchSnapshot();
  });

  it("remove_call_transform should work with import * as", async () => {
    let code = `
import * as React from "react";
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

    const napi_res = transform.transformBabelImport(code, {
      removeUseEffect: true,
    });

    expect(napi_res.code).toMatchSnapshot();
  });

  it("remove_call_transform should work with multi import", async () => {
    let code = `
import * as React from "react";
import ReactDOM from "react-dom";
import ReactDefault, { useEffect } from "react";
import { useEffect as useEffect2 } from "react";
import * as AnotherReact from "react";

function App() {
    const [num, setNum] = React.useState(1);
    React.useState(2);
    
    React.useEffect(() => {
        setNum(2);
    }, []);

    useEffect(() => {
        setNum(3);
    }, []);

    useEffect2(() => {
        setNum(3);
    }, []);

    AnotherReact.useEffect(() => {
        setNum(4);
    }, []);

    ReactDefault.useEffect(() => {
        setNum(5);
    }, []);

    return <div>{num}</div>;
}
ReactDOM.render(<Page/>, document.getElementById("root"));
`;

    const napi_res = transform.transformBabelImport(code, {
      removeUseEffect: true,
    });

    expect(napi_res.code).toMatchSnapshot();
  });

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

    const napi_res = transform.transformBabelImport(code, {
      removeUseEffect: true,
    });

    expect(napi_res.code).toMatchSnapshot();
  });

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

    const napi_res = transform.transformBabelImport(code, {
      removeUseEffect: true,
    });

    const consumer = await new SourceMapConsumer(napi_res.map as any);

    const position1 = consumer.originalPositionFor({
      line: 5,
      column: 4,
    });
    const position2 = consumer.originalPositionFor({
      line: 6,
      column: 12,
    });
    const position3 = consumer.originalPositionFor({
      line: 8,
      column: 1,
    });

    expect(napi_res.code).toMatchSnapshot();
    expect(position1.line).toMatchSnapshot();
    expect(position1.column).toMatchSnapshot();
    expect(position2.line).toMatchSnapshot();
    expect(position2.column).toMatchSnapshot();
    expect(position3.line).toMatchSnapshot();
    expect(position3.column).toMatchSnapshot();
  });
});

describe("speedy-napi: code type config", () => {
  it("can parse ts only syntax", () => {
    // https://github.com/speedy-js/speedy-native/issues/36
    const code = `
import { useEffect, useState } from "react";

function useCount() {
  const [count, setCount] = useState(0);
  useEffect(() => {
    console.log(count);
  }, [count]);

  return [count, setCount];
}

const useName = <[]>useCount();
`;

    const res = transform.transformBabelImport(code, {
      removeUseEffect: true,
      codeType: "ts",
    });

    expect(res.code).toMatchSnapshot();
  });

  it("parse will error if wrongly set tsx", () => {
    const code = `
import { useEffect, useState } from "react";

function useCount() {
  const [count, setCount] = useState(0);
  useEffect(() => {
    console.log(count);
  }, [count]);

  return [count, setCount];
}

const useName = <[]>useCount();
`;

    expect(() =>
      transform.transformBabelImport(code, {
        removeUseEffect: true,
        codeType: "tsx",
      })
    ).toThrowErrorMatchingInlineSnapshot(
      `"Unexpected token \`[\`. Expected jsx identifier"`
    );
  });

  it("default will parse tsx", () => {
    const code = `
import { useEffect, useState } from "react";

function useCount() {
  const [count, setCount] = useState(0);
  useEffect(() => {
    console.log(count);
  }, [count]);

  return [count, setCount];
}

const useName = <[]>useCount();`;

    expect(() =>
      transform.transformBabelImport(code, {
        removeUseEffect: true,
      })
    ).toThrowErrorMatchingInlineSnapshot(
      `"Unexpected token \`[\`. Expected jsx identifier"`
    );
  });

  it("parse ts code will error if wrongly set js", () => {
    const code = `
import { useEffect } from "react";

function useCount(): void {
  useEffect(() => {
  }, []);
}

const useName = useCount();
`;

    expect(() =>
      transform.transformBabelImport(code, {
        removeUseEffect: true,
        codeType: "js",
      })
    ).toThrowErrorMatchingInlineSnapshot(`"Expected '{', got ':'"`);
  });

  it("parse jsx code will error if wrongly set js", () => {
    const code = `
import { useEffect } from "react";

function useCount() {
  return <div />
}

const useName = useCount();
`;

    expect(() =>
      transform.transformBabelImport(code, {
        removeUseEffect: true,
        codeType: "js",
      })
    ).toThrowErrorMatchingInlineSnapshot(
      `"Unexpected token \`>\`. Expected this, import, async, function, [ for array literal, { for object literal, @ for decorator, function, class, null, true, false, number, bigint, string, regexp, \` for template literal, (, or an identifier"`
    );
  });
});
