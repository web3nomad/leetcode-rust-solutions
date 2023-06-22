# LeetCode Rust Solutions

## config VSCode

`settings.json` 里面修改成如下，符合 Rust 包名的规则

```json
"leetcode.filePath": {
    "default": {
        "folder": "solutions",
        // "filename": "${id}.${kebab-case-name}.${ext}"
        "filename": "sol_${id}_${snake_case_name}.${ext}"
    }
}
```

## Compile code with `rustc`

```shell
rustc main.rs --out-dir output
```

## Run code with `rustc`

```shell
./ourput/main
```
