# CLAUDE.md

本文件为 Claude Code (claude.ai/code) 在此仓库中工作时提供指导。

## 项目概述

Synton (Synthetic Token Language，合成Token语言) 是一个研究项目，探索专为 AI/LLM 编程而非人类可读性优化的编程语言设计。这是一个处于概念/研究阶段的纯文档项目。

### 核心问题

传统编程语言（Python、Java、C++）是为人类认知设计的，强调可读性、助记符变量名和对硬件的抽象。而 LLM 具有不同的约束和优势：

| 人类开发者 | AI 开发者 |
| ----------- | ---------- |
| 记忆力有限，逻辑深度有限 | 上下文窗口有限，逻辑深度极强 |
| 偏好自然语言般的语法 | 偏好结构化数据（AST/图）、确定性 |
| 错误：语法、逻辑疏忽、拼写 | 错误：幻觉、语义歧义 |
| 阅读错误日志，手动调试 | 需要结构化 JSON/状态转储用于自我修正 |
| 需要注释和空白 | 冗余浪费 Token |

### 核心设计哲学

"形式化验证优先，Token 密度最大化，反馈结构化。"

## 架构概览

Synton 提议了一个多层架构：

1. 表示层：压缩的 AST 序列化格式，优化 Token 效率
2. 约束层：基于契约的类型系统，结合 SMT 求解器（如 Z3）在编译期防止幻觉
3. 运行时层：混合执行引擎（确定性 + 神经网络），支持沙盒执行
4. 人机接口层：实时反编译为 Python/TypeScript 供人类监督

### 核心语言特性

- Token 高效语法：增强型波兰表示法（前缀表达式）- `+ a b` 而非 `a + b`
- 基于契约的类型：带逻辑断言的类型 - `int { val > 0 && val < 150 }`
- 确定性引用：通过哈希进行内容寻址导入 - `import hash:sha256:e3b0c442...`
- 概率原语：一等公民的不确定性处理 - `maybe<T, confidence>` 类型
- 结构化执行反馈：JSON 调试状态对象而非文本堆栈跟踪
- 语义导入：基于哈希或嵌入向量的库引用

## 语法示例对比

Python（人类可读）：

```python
def fib(n: int) -> int:
    if n <= 1: return n
    return fib(n-1) + fib(n-2)
```

Synton（AI 优化）：

```lisp
!fn:fib [n:i32] -> i32
  @pre(n >= 0)           // 前置条件契约
  @post($ret >= n)       // 后置条件契约
  (branch (<= n 1)
    n
    (+ (call:fib (- n 1)) (call:fib (- n 2)))
  )
```

## 结构化错误协议

当运行时发生错误时，Synton 返回调试状态对象（DSO）：

```json
{
  "status": "RuntimeError",
  "error_code": "CONSTRAINT_VIOLATION",
  "location": "node_id:45a",
  "context": {
    "variable_values": {"%1": 0, "%2": -5},
    "memory_snapshot": "hash:..."
  },
  "expected": "val > 0",
  "actual": "-5"
}
```

这种结构化的 JSON 使 AI 无需解析自然语言错误消息即可进行自我修正。

## 开发流程

这是一个文档仓库，具有以下特点：

- 无构建系统、测试或 CI/CD
- Git 已初始化但尚无提交
- 主要产物：`README.md` 包含完整技术规范

贡献项目时请注意：

- 文档以中英文双语维护
- 规范涵盖设计哲学、语法、类型系统、编译器架构和示例工作流
- 代码示例同时提供 Python（用于对比）和 Synton 语法
