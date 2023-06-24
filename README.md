# LeetCode Rust Solutions

## 项目设置

项目使用的插件是 [LeetCode](https://marketplace.visualstudio.com/items?itemName=LeetCode.vscode-leetcode)

### 配置 vscode

`settings.json` 里面修改成如下，符合 Rust 包名的规则

```json
"leetcode.filePath": {
    "default": {
        "folder": "solutions",
        "filename": "sol_${id}_${snake_case_name}.${ext}"
    }
}
```

这时候每次点开一个题目选择 Code Now 的时候，会在 `solutions` 目录下自动生成一个上面命名规则的解题文件，比如 `sol_1_two_sum.rs`，提交的时候也会自动来寻找这个文件。

### 修改 solution 文件

详见 `solutions` 目录下的文件，比如 `sol_1_two_sum.rs`，在文件顶上添加，提交到 leetcode 的时候，插件只会取 `// @lc code=start` 到 `// @lc code=end` 之间的代码。

```rust
pub struct Solution;
```

### 修改 `solutions/mod.rs` 文件

将每个题目的名称添加到 `mod.rs` 中，将每个 solution 作为模块导出，方便被 rustc 和 cargo 使用，比如：

```rust
pub mod sol_1_two_sum;
pub mod sol_2_add_two_numbers;
// ...
```

## 本地调试

### 使用 rustc 直接编译和运行

```shell
# compile
rustc main.rs --out-dir output
# run
./ourput/main
```

### 使用 cargo 运行测试脚本（推荐）

```shell
cargo test
```

每个 solution 文件单独添加测试用例，比如 `sol_1_two_sum.rs`，在文件最后添加：

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        // 编写测试脚本，使用 assert_eq! 等宏进行检验
    }
}
```
