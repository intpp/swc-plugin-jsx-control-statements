const assert = require("node:assert/strict");
const { join } = require("node:path");
const { runInNewContext } = require("node:vm");
const { transformFileSync } = require("@swc/core");
const { renderToStaticMarkup } = require("react-dom/server");

const PLUGIN = "swc-plugin-jsx-control-statements";
const EXAMPLE = join(__dirname, "example.tsx");
const OPTIONS = {
    jsc: {
        target: "es2022",
        parser: { syntax: "typescript", tsx: true },
        transform: { react: { runtime: "automatic" } },
        experimental: { plugins: [[PLUGIN, {}]] },
    },
    module: { type: "commonjs" },
};

const { code } = transformFileSync(EXAMPLE, OPTIONS);
const output = {};
runInNewContext(code, { exports: output, require });
const CONTENT = "<b>0Ada</b><b>1Grace</b><i>ADA0</i><i>GRACE1</i>" +
    "<span>inner</span><span>INNER</span><span>ada,grace</span><span>outer</span>";
assert.equal(renderToStaticMarkup(output.Example({ enabled: true })), `<main>onyes${CONTENT}</main>`);
assert.equal(renderToStaticMarkup(output.Example({ enabled: false })), `<main>offno${CONTENT}</main>`);

const [first, second, text] = output.unicode();
const SURROGATE = "\uD800";
assert.equal(first.key, `${SURROGATE}-0`);
assert.equal(second.key, `${SURROGATE}-1`);
assert.equal(text, SURROGATE);

const INDEX_LIST = "<em>index</em><i>outer0</i><i>outer1</i>!!";
assert.equal(renderToStaticMarkup(output.IndexList()), INDEX_LIST);
