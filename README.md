# stormbreaker

## 命令

### dump
从数据库dump下表结构信息，并保存在文件中
```bash
storm dump -c config.yaml -e env1
```

### compare
比较两个环境的表结构差异，生成从env1到env2的sql文件
```bash
storm compare -c config.yaml -e env2 -f file1
```