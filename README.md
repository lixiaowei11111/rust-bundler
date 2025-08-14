rust-bundler/
├── Cargo.toml                    # workspace 根配置
├── README.md
├── LICENSE
├── .github/
│   └── workflows/
│       ├── ci.yml
│       └── release.yml
├── examples/                     # 示例项目
│   ├── basic/
│   ├── typescript/
│   └── react/
├── benchmarks/                   # 性能基准测试
│   ├── Cargo.toml
│   └── src/
├── xtask/                        # 构建任务
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── build.rs
│       ├── test.rs
│       ├── bench.rs
│       ├── release.rs
│       └── utils/
├── crates/                       # 所有子包
│   ├── rust_bundler/             # 主包，重新导出所有API
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs
│   ├── bundler_core/             # 核心编译器
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── compilation/
│   │       │   ├── mod.rs
│   │       │   ├── compiler.rs
│   │       │   ├── stats.rs
│   │       │   └── cache.rs
│   │       ├── module/
│   │       │   ├── mod.rs
│   │       │   ├── module.rs
│   │       │   ├── dependency.rs
│   │       │   ├── module_graph.rs
│   │       │   └── chunk_graph.rs
│   │       ├── build/
│   │       │   ├── mod.rs
│   │       │   ├── make.rs
│   │       │   ├── seal.rs
│   │       │   └── emit.rs
│   │       └── error/
│   │           ├── mod.rs
│   │           └── bundler_error.rs
│   ├── bundler_config/           # 配置系统
│   │   └── ...
│   ├── bundler_resolver/         # 模块解析（参考enhanced-resolve）
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── resolver.rs
│   │       ├── description_file.rs
│   │       ├── path_resolver.rs
│   │       └── plugins/
│   │           ├── mod.rs
│   │           ├── alias_plugin.rs
│   │           ├── main_field_plugin.rs
│   │           └── extension_plugin.rs
│   ├── bundler_loader/           # 加载器系统
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── runner/
│   │       │   ├── mod.rs
│   │       │   ├── loader_runner.rs
│   │       │   └── loader_context.rs
│   │       ├── builtin/
│   │       │   ├── mod.rs
│   │       │   ├── js_loader.rs
│   │       │   ├── ts_loader.rs
│   │       │   ├── css_loader.rs
│   │       │   └── asset_loader.rs
│   │       └── swc/
│   │           ├── mod.rs
│   │           └── transformer.rs
│   ├── bundler_plugin_system/    # 插件架构
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── plugin.rs
│   │       ├── plugin_driver.rs
│   │       ├── hooks/
│   │       │   ├── mod.rs
│   │       │   ├── compilation_hooks.rs
│   │       │   ├── compiler_hooks.rs
│   │       │   └── hook_context.rs
│   │       └── tapable/          # 类似webpack的tapable
│   │           ├── mod.rs
│   │           ├── sync_hook.rs
│   │           ├── async_hook.rs
│   │           └── bail_hook.rs
│   ├── bundler_plugin/           # 内置插件
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── html_plugin.rs
│   │       ├── define_plugin.rs
│   │       ├── banner_plugin.rs
│   │       ├── copy_plugin.rs
│   │       └── progress_plugin.rs
│   ├── bundler_optimizer/        # 优化器
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── chunk/
│   │       │   ├── mod.rs
│   │       │   ├── split_chunks.rs
│   │       │   └── chunk_ids.rs
│   │       ├── tree_shaking/
│   │       │   ├── mod.rs
│   │       │   ├── mark.rs
│   │       │   └── shake.rs
│   │       ├── minify/
│   │       │   ├── mod.rs
│   │       │   ├── terser.rs
│   │       │   └── swc_minify.rs
│   │       └── concatenate/
│   │           ├── mod.rs
│   │           └── module_concatenation.rs
│   ├── bundler_hash/             # 哈希和缓存
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── xxhash.rs
│   │       └── persistent_cache.rs
│   ├── bundler_utils/            # 工具库
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── fs.rs
│   │       ├── path.rs
│   │       ├── source_map.rs
│   │       ├── identifier.rs
│   │       └── swc_helpers.rs
│   ├── bundler_cli/              # CLI
│   │   └── ...
│   ├── bundler_dev_server/       # 开发服务器
│   │   └── ...
│   └── bundler_binding/          # 绑定层
│       ├── node/                 # Node.js绑定
│       └── wasm/                 # WASM绑定
└── tests/
    ├── fixtures/
    ├── integration/
    └── snapshots/