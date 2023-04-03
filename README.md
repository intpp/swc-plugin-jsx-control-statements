## 🦀 JSX control statements for swc

Original idea: [babel-plugin-jsx-control-statements](https://github.com/AlexGilleran/jsx-control-statements)


#### Usage

```javascript
jsc: {
    experimental: {
        plugins: [
            ['swc-plugin-jsx-control-statements', {}],
        ],
    },
},
```


TODO:
- [ ] Support `<For />` tag
- [ ] Throw errors for unknown props
