#### Workspace

使用工作区管理多个项目，可以包含库程序或二进制程序。

1. 创建工作区
   ```shell
   mkdir workspace
   touch Cargo.toml
   ```
   
    定义工作区下包含的项目目录名称
   ```toml
   # Cargo.toml
   [workspace]
   members = ["app", "sum_crate"]
   ```
   
2. 创建项目 
   1. 库程序:`cargo new sum_crate --lib --vcs=none`
   2. 二进制程序:`cargo new app --vcs=none`
   
3. 使用依赖库程序项目
    
   ```toml
   # app/Cargo.toml
   [dependencies]
   sum_crate = { path = "../sum_crate" }
   ```
   设置依赖`sum_crate`库为本地依赖路径