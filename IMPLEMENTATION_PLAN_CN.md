# Synton 语言实施计划
**面向 AI 原生编程语言的生产级设计**

**状态：** 规划阶段 - 等待批准
**创建日期：** 2025-02-01
**范围：** 生产部署的核心语言特性

---

## 执行摘要

本计划详细阐述了生产就绪的 Synton 语言四个核心特性的设计与实现：

1. **类型系统与契约** - 基于 SMT 求解器的约束验证
2. **AST 表示与解析** - Token 高效的语法和序列化
3. **概率原语** - 使用 `maybe<T>` 类型的一等不确定性处理
4. **结构化错误协议** - 基于 JSON 的调试状态对象（DSO）

**生产目标：**
- 形式化验证正确性：99.9%+ 的约束检测率
- Token 效率：相比 Python 等效代码减少 40-60%
- 常见操作的子毫秒级约束验证
- 人类可读的反编译，100% 语义保持

---

## 需求重述

### 功能需求

**FR1: 带契约的类型系统**
- 定义带逻辑谓词的精化类型：`T where P`
- 集成 Z3 SMT 求解器进行编译期验证
- 支持前置条件（`@pre`）、后置条件（`@post`）和不变量
- 契约传播的类型推断
- 约束违规的清晰错误报告

**FR2: AST 与解析器**
- 压缩的 S 表达式语法（波兰表示法）
- 直接的 AST 到二进制序列化格式
- Token 高效编码（目标：简单函数 <15 tokens）
- 双向反编译为 Python/TypeScript
- 通过哈希引用的内容寻址导入

**FR3: 概率类型**
- `maybe<T, confidence>` 类型，自动阈值处理
- 带置信度分数的模糊匹配原语
- 作为一等公民的张量操作
- 带置信度传播的概率分支

**FR4: 结构化错误**
- 基于 JSON 的错误对象（DSO - 调试状态对象）
- 错误时完整的程序状态快照
- AST 节点 ID 跟踪，精确错误定位
- 机器可读的修正建议
- 错误上下文的人类可读反编译

### 非功能需求

**NFR1: 性能**
- 约束检查：简单谓词 <1ms，复杂 Z3 查询 <100ms
- 解析速度：>1MB/s
- 二进制 AST 大小：源文本大小的 <30%

**NFR2: 正确性**
- 契约违规零误报
- 错误 DSO 的完整状态捕获
- 确定性解析（无歧义）

**NFR3: 开发体验**
- 面向 AI 和人类的清晰错误消息
- 带实时反编译的 IDE 支持
- 全面的测试覆盖（95%+）

---

## 实施阶段

### 阶段 1: 基础 - AST 与解析器（第 1-3 周）

**目标：** 构建所有其他特性依赖的核心语法基础设施。

#### 1.1 语法定义
- 为 Synton 语法定义正式的 EBNF 语法
- 指定 token 类别：关键字、操作符、字面量、标识符
- 文档化优先级和结合性规则
- 创建语法正确性测试套件

**交付物：**
- `docs/grammar.md` - 完整的 EBNF 规范
- `test/samples/valid/` - 50+ 个有效语法示例
- `test/samples/invalid/` - 30+ 个无效语法示例

#### 1.2 词法分析器实现
```typescript
// 目标结构（TypeScript）
class Lexer {
  tokenize(source: string): Token[] {
    // 产生压缩的 token：
    // - 单字符操作符：+、-、*、/ 等
    // - 多字符操作符：->、<=、>=、==
    // - 关键字：fn、branch、call 等
    // - 字面量：整数、浮点数、字符串
  }

  // 优化：Token ID 映射
  // 关键字 -> 单字节 ID（0-255）
  // 操作符 -> 单字节 ID
  // 标识符 -> 压缩为变量引用
}
```

**关键决策：**
- 使用基于字节的 token 编码以实现最大密度
- 为操作符/关键字保留前 64 个 ID
- 仅在字符串字面量中支持 unicode

#### 1.3 解析器实现
```typescript
class Parser {
  parse(tokens: Token[]): AST {
    // 使用递归下降构造 AST
    // 解析期间验证结构
    // 为每个节点附加源位置元数据
  }

  // 波兰表示法解析
  parsePrefixExpression(): ASTNode {
    // (+ a b) -> BinaryOp(op='+', left=a, right=b)
    // (branch condition true_branch false_branch)
  }
}
```

**关键决策：**
- AST 节点作为不可变结构
- 每个节点携带：类型、子节点、位置、元数据
- 支持解析和序列化

#### 1.4 二进制 AST 格式
```typescript
// 二进制序列化格式
class ASTSerializer {
  serialize(ast: AST): Uint8Array {
    // 格式：
    // [4 字节魔数] [版本] [节点数]
    // [节点表...] // 每个节点：type_id + children_indices
    // [字面量表...] // 字符串、数字
    // [元数据...] // 源位置、契约
  }

  deserialize(binary: Uint8Array): AST {
    // 以完整保真度重建 AST
  }
}
```

**压缩目标：**
- 节点类型：1 字节（256 种节点类型）
- 子节点索引：varint（基于大小 1-5 字节）
- 字符串字面量：UTF-8 去重

#### 1.5 反解析器（人类视图）
```typescript
class Decompiler {
  toPython(ast: AST): string {
    // 生成 Python 等效代码
    // 保持语义，提高可读性
  }

  toTypeScript(ast: AST): string {
    // 生成带类型的 TypeScript
  }
}
```

**成功标准：**
- ✓ 所有有效样本正确解析
- ✓ 二进制格式达到 70%+ 压缩率
- ✓ 往返：解析 → 序列化 → 反序列化 → 反编译保持语义
- ✓ 反编译产生有效的 Python/TypeScript

---

### 阶段 2: 类型系统与契约（第 4-7 周）

**目标：** 构建形式化验证系统以防止 AI 幻觉。

#### 2.1 类型表示
```typescript
// 类型系统核心
type Type =
  | PrimitiveType    // i32、f64、bool、string
  | NamedType        // 用户定义类型
  | RefinementType   // T where predicate
  | MaybeType        // maybe<T, confidence>
  | TensorType       // tensor<T>[dimensions]
  | FunctionType     // [args] -> ret

interface RefinementType {
  base: Type;
  predicate: AST;    // 逻辑表达式
  variables: string[]; // 谓词中的自由变量
}

interface Contract {
  preconditions: AST[];   // @pre 子句
  postconditions: AST[];  // @post 子句
  invariants: AST[];      // @inv 子句
}
```

#### 2.2 类型检查器
```typescript
class TypeChecker {
  check(ast: AST, env: TypeEnv): TypingResult {
    // 推断所有表达式的类型
    // 验证契约格式正确
    // 检查函数调用兼容性
    // 收集所有约束以供验证
  }

  // 示例约束收集
  collectConstraints(fn: FunctionNode): Constraint[] {
    // 从：@pre(n >= 0)
    // 约束：对于 fn 的所有调用，arg[0] >= 0

    // 从：@post($ret >= n)
    // 约束：在返回时，result >= arg[0]
  }
}
```

#### 2.3 SMT 求解器集成（Z3）
```typescript
class ConstraintVerifier {
  private solver: Z3.Solver;

  verifyConstraints(constraints: Constraint[]): VerifyResult {
    // 1. 将 Synton 谓词转换为 Z3 格式
    const z3Asserts = constraints.map(c => this.toZ3(c));

    // 2. 断言所有约束
    this.solver.assert(z3Asserts);

    // 3. 检查可满足性
    const result = this.solver.check();

    if (result === "unsat") {
      // 未找到反例 - 约束可能不一致
      return {
        status: "invalid",
        reason: "没有可能的值满足所有约束"
      };
    }

    if (result === "sat") {
      // 找到满足赋值 - 约束一致
      const model = this.solver.getModel();

      // 检查是否存在潜在违规
      const counterexamples = this.findCounterexamples(constraints);

      if (counterexamples.length > 0) {
        return {
          status: "violated",
          counterexamples,
          // 示例：
          // {
          //   constraint: "@post($ret >= n)",
          //   inputs: {n: -1},
          //   output: -2,
          //   reason: "-2 < -1"
          // }
        };
      }

      return {status: "verified"};
    }

    return {status: "unknown"};
  }

  private toZ3(predicate: AST): Z3.Ast {
    // 转换：(>= n 0)
    // 到：Z3.mkGe(n_var, Z3.mkInt(0))

    // 处理：(&& (> x 0) (< x 100))
    // 到：Z3.mkAnd(
    //        Z3.mkGt(x_var, Z3.mkInt(0)),
    //        Z3.mkLt(x_var, Z3.mkInt(100))
    //      )
  }
}
```

#### 2.4 契约推断
```typescript
class ContractInference {
  inferContracts(fn: FunctionNode): Contract {
    // 分析函数体以推断：
    // - 空检查 -> @pre(x != null)
    // - 除法操作 -> @pre(denominator != 0)
    // - 数组访问 -> @pre(index >= 0 && index < length)
    // - 返回值使用 -> @post($ret != null)

    // 使用符号执行查找潜在违规
  }
}
```

#### 2.5 与解析器集成
```typescript
// 使用契约信息扩展 AST 节点
interface FunctionNode {
  name: string;
  params: Param[];
  returnType: Type;
  body: AST;
  contract: Contract;  // 新增：附加的契约
  inferredContract?: Contract;  // 自动推断
}
```

**成功标准：**
- ✓ 在测试套件中检测 99%+ 的约束违规
- ✓ 对于 <10 个约束的函数，<100ms 内验证契约
- ✓ 生成带反例的有用错误消息
- ✓ 支持算术、逻辑和数组谓词
- ✓ 处理数组的全称/存在量词

---

### 阶段 3: 概率原语（第 6-9 周）

**注意：** 与阶段 2 后期并行运行。

#### 3.1 Maybe 类型实现
```typescript
interface MaybeType {
  type: "maybe";
  inner: Type;
  confidence: number | "dynamic";  // 固定或运行时计算
}

// 运行时表示
class Maybe<T> {
  constructor(
    public value: T | null,
    public confidence: number,  // 0.0 到 1.0
    public metadata?: Record<string, any>
  ) {}

  isCertain(threshold = 0.95): boolean {
    return this.confidence >= threshold;
  }

  unwrap(): T {
    if (this.value === null) {
      throw new UncertaintyError(this.confidence);
    }
    return this.value;
  }

  map<U>(fn: (T) => U): Maybe<U> {
    if (this.value === null) {
      return new Maybe(null, this.confidence * 0.9);  // 衰减
    }
    try {
      return new Maybe(fn(this.value), this.confidence);
    } catch {
      return new Maybe(null, 0.0);
    }
  }
}
```

#### 3.2 模糊操作
```typescript
// 内置模糊原语
class FuzzyOps {
  // 带置信度的字符串相似度
  static fuzzyMatch(
    pattern: string,
    text: string
  ): Maybe<string> {
    const similarity = levenshteinSimilarity(pattern, text);
    return new Maybe(
      similarity > 0.8 ? text : null,
      similarity
    );
  }

  // 语义搜索（向量相似度）
  static semanticSearch(
    query: Vector,
    candidates: Vector[]
  ): Maybe<number> {  // 最佳匹配的索引
    const similarities = candidates.map(v =>
      cosineSimilarity(query, v)
    );
    const maxSim = Math.max(...similarities);
    const bestIdx = similarities.indexOf(maxSim);

    return new Maybe(
      maxSim > 0.85 ? bestIdx : null,
      maxSim
    );
  }

  // 概率选择
  static amb<T>(options: Maybe<T>[]): Maybe<T> {
    // 返回置信度最高的选项
    const best = options.reduce((a, b) =>
      a.confidence > b.confidence ? a : b
    );
    return best;
  }
}
```

#### 3.3 张量原语
```typescript
// 原生张量操作
class Tensor<T> {
  constructor(
    public data: T[],
    public shape: number[]
  ) {}

  // 逐元素操作
  add(other: Tensor<T>): Tensor<T> { /* ... */ }
  mul(other: Tensor<T>): Tensor<T> { /* ... */ }

  // 线性代数
  matmul(other: Tensor<T>): Tensor<T> { /* ... */ }

  // 聚合
  sum(): T { /* ... */ }
  mean(): T { /* ... */ }

  // 重塑
  reshape(newShape: number[]): Tensor<T> { /* ... */ }
}
```

#### 3.4 置信度传播
```typescript
class ConfidencePropagator {
  // 通过操作跟踪置信度
  visitBinaryOp(op: BinaryOp): number {
    switch (op.operator) {
      case "+":
      case "*":
        // 对于算术：min(置信度)
        return Math.min(op.left.conf, op.right.conf);
      case "&&":
        // 逻辑与：乘积
        return op.left.conf * op.right.conf;
      case "||":
        // 逻辑或：max + 调整
        return Math.max(op.left.conf, op.right.conf);
    }
  }

  visitCall(fn: FunctionCall): number {
    // 函数输出置信度基于：
    // - 最小参数置信度
    // - 函数的固有可靠性
    const argConf = Math.min(...fn.args.map(a => a.conf));
    const fnReliability = fn.metadata.reliability ?? 1.0;
    return argConf * fnReliability;
  }
}
```

#### 3.5 与类型系统集成
```typescript
// 扩展 maybe 类型的类型检查
class TypeChecker {
  checkMaybeOp(node: MaybeOp): Type {
    // maybe<T> 操作必须保持置信度
    // 示例：
    // (def result (fuzzy_match pattern text))
    // result: maybe<string, dynamic>

    // 检查下游使用
    if (node.isForcedUnwrap && !node.hasGuard) {
      this.warning("强制解包但未检查置信度");
    }
  }
}
```

**成功标准：**
- ✓ Maybe 类型操作开销 <5%
- ✓ 置信度跟踪是单调的（不会无故增加）
- ✓ 模糊操作在测试数据集上达到 >90% 精确度
- ✓ 张量操作支持高达 4D 的维度
- ✓ 与契约集成：`maybe<T> where confidence > 0.9`

---

### 阶段 4: 结构化错误协议（第 8-10 周）

**目标：** 创建机器可读的错误格式，用于 AI 自我修正。

#### 4.1 调试状态对象（DSO）模式
```typescript
interface DebugStateObject {
  // 错误分类
  status: "CompileError" | "RuntimeError" | "ConstraintViolation";
  error_code: string;  // 机器可读枚举
  severity: "error" | "warning" | "info";

  // 位置
  location: {
    node_id: string;      // AST 节点标识符
    source_span?: {       // 人类可读位置
      file: string;
      start_line: number;
      start_col: number;
      end_line: number;
      end_col: number;
    };
  };

  // 状态快照
  context: {
    variable_values: Record<string, any>;  // 所有作用域内变量
    call_stack: StackFrame[];              // 活动函数调用
    memory_snapshot: string;               // 堆状态哈希
    contract_states: ContractState[];      // 活动契约
  };

  // 错误详情
  error_details: {
    expected: any;     // 期望的内容（类型/值）
    actual: any;       // 实际发现的内容
    reason: string;    // 人类可读解释
  };

  // 修正提示
  suggestions: Suggestion[];
}

interface StackFrame {
  function_name: string;
  node_id: string;
  locals: Record<string, any>;
  line_number: number;
}

interface ContractState {
  contract: string;      // 契约源码
  is_satisfied: boolean;
  counterexample?: any;
}

interface Suggestion {
  type: "fix" | "hint" | "refactor";
  description: string;
  code?: string;         // 建议的代码补丁
  confidence: number;
}
```

#### 4.2 错误生成

**编译错误：**
```typescript
class Compiler {
  generateError(error: CompilationError): DSO {
    return {
      status: "CompileError",
      error_code: error.code,
      severity: "error",
      location: {
        node_id: error.node.id,
        source_span: this.getLocation(error.node)
      },
      context: {
        variable_values: {},
        call_stack: [],
        memory_snapshot: "",
        contract_states: []
      },
      error_details: {
        expected: error.expected,
        actual: error.actual,
        reason: error.message
      },
      suggestions: this.suggestFixes(error)
    };
  }

  suggestFixes(error: CompilationError): Suggestion[] {
    // 示例：类型不匹配
    if (error.code === "TYPE_MISMATCH") {
      return [{
        type: "fix",
        description: `添加类型转换：${error.actual} -> ${error.expected}`,
        code: `(cast ${error.actual} ${error.expected})`,
        confidence: 0.95
      }];
    }

    // 示例：未定义变量
    if (error.code === "UNDEFINED_VAR") {
      return [{
        type: "hint",
        description: `变量 '${error.varName}' 未定义。定义它或从库导入。`,
        confidence: 1.0
      }, {
        type: "fix",
        description: `在库中搜索相似名称`,
        code: `(import sem:"${error.varName}")`,
        confidence: 0.7
      }];
    }

    return [];
  }
}
```

**运行时错误：**
```typescript
class Runtime {
  handleError(error: RuntimeError): DSO {
    // 捕获完整程序状态
    const state = this.captureState();

    return {
      status: "RuntimeError",
      error_code: error.code,
      severity: "error",
      location: {
        node_id: error.node.id,
        source_span: this.getSourceLocation(error.node)
      },
      context: state,
      error_details: {
        expected: error.expected,
        actual: error.actual,
        reason: error.message
      },
      suggestions: this.generateSuggestions(error)
    };
  }

  private captureState() {
    return {
      variable_values: this.vm.getAllVariables(),
      call_stack: this.vm.getCallStack().map(frame => ({
        function_name: frame.functionName,
        node_id: frame.nodeId,
        locals: frame.locals,
        line_number: frame.lineNumber
      })),
      memory_snapshot: this.vm.hashMemory(),
      contract_states: this.vm.getActiveContracts().map(c => ({
        contract: c.source,
        is_satisfied: this.checkContract(c),
        counterexample: this.findCounterexample(c)
      }))
    };
  }
}
```

**约束违规：**
```typescript
class ConstraintVerifier {
  generateViolationDSO(
    constraint: Contract,
    counterexample: Counterexample
  ): DSO {
    return {
      status: "ConstraintViolation",
      error_code: "CONTRACT_FAILED",
      severity: "error",
      location: {
        node_id: constraint.nodeId,
        source_span: this.getLocation(constraint.node)
      },
      context: {
        variable_values: counterexample.values,
        call_stack: [],
        memory_snapshot: "",
        contract_states: [{
          contract: constraint.source,
          is_satisfied: false,
          counterexample: counterexample
        }]
      },
      error_details: {
        expected: constraint.predicate,
        actual: this.formatActual(counterexample),
        reason: `在 ${constraint.location} 违反约束`
      },
      suggestions: [{
        type: "fix",
        description: `加强前置条件或削弱后置条件`,
        code: `@pre(${counterexample.suggestion})`,
        confidence: 0.85
      }]
    };
  }
}
```

#### 4.3 DSO 序列化
```typescript
class DSOFormatter {
  toJSON(dso: DebugStateObject): string {
    return JSON.stringify(dso, null, 2);
  }

  toHumanReadable(dso: DebugStateObject): string {
    // 为人类消费生成格式化文本
    // 示例：
    //
    // Error: ConstraintViolation
    //   Location: fib.synton:5:10
    //
    //   Contract violated: @post($ret >= n)
    //
    //   Counterexample:
    //     n = -1
    //     expected: ret >= -1
    //     actual: ret = -2
    //
    //   Suggestion:
    //     Change @pre(n >= 0) to @pre(n >= 1)
    //
    //   Stack trace:
    //     at fib (fib.synton:5)
    //     at main (main.synton:12)
  }

  toAIReadable(dso: DebugStateObject): string {
    // 为 LLM 消费优化的紧凑格式
    // 移除冗余信息，专注于可操作数据
    return `ERR:${dso.error_code}|LOC:${dso.location.node_id}|EXP:${dso.error_details.expected}|ACT:${dso.error_details.actual}`;
  }
}
```

#### 4.4 错误恢复系统
```typescript
class ErrorRecovery {
  async attemptFix(
    error: DSO,
    sourceCode: string,
    maxAttempts = 3
  ): Promise<Result> {
    for (let i = 0; i < maxAttempts; i++) {
      // 1. 选择最佳建议
      const suggestion = this.selectSuggestion(error);

      if (!suggestion) {
        break;  // 没有可用的自动修复
      }

      // 2. 应用补丁
      const patchedCode = this.applyPatch(sourceCode, suggestion.code);

      // 3. 重新编译/验证
      const result = await this.tryCompile(patchedCode);

      if (result.success) {
        return {
          success: true,
          code: patchedCode,
          attempts: i + 1,
          appliedSuggestion: suggestion
        };
      }

      // 4. 如果仍然失败，更新错误并重试
      error = result.error;
    }

    return {success: false, error};
  }

  private selectSuggestion(error: DSO): Suggestion | null {
    // 按置信度排序，返回最高的
    return error.suggestions
      .sort((a, b) => b.confidence - a.confidence)[0] ?? null;
  }
}
```

**成功标准：**
- ✓ 所有错误产生带有完整状态的有效 DSO
- ✓ 典型程序的 DSO 大小 <100KB（复杂程序 <1MB）
- ✓ 建议 >80% 的情况下可操作
- ✓ AI 可读格式相比完整 DSO 减少 70% token 计数
- ✓ 人类可读格式清晰可操作

---

### 阶段 5: 集成与测试（第 11-13 周）

#### 5.1 端到端集成
```typescript
class SyntonPipeline {
  async compile(source: string): Promise<CompilationResult> {
    // 1. 词法分析
    const tokens = this.lexer.tokenize(source);

    // 2. 解析 AST
    const ast = this.parser.parse(tokens);

    // 3. 类型检查
    const typing = this.typeChecker.check(ast);

    // 4. 契约验证
    const constraints = this.constraintVerifier.collectConstraints(ast);
    const verification = this.constraintVerifier.verifyConstraints(constraints);

    if (!verification.verified) {
      return {
        success: false,
        errors: verification.violations.map(v =>
          this.constraintVerifier.generateViolationDSO(v)
        )
      };
    }

    // 5. 二进制序列化
    const binary = this.serializer.serialize(ast);

    return {
      success: true,
      binary,
      ast,
      typing
    };
  }

  async execute(binary: Uint8Array, inputs: any[]): Promise<ExecutionResult> {
    // 1. 反序列化
    const ast = this.serializer.deserialize(binary);

    // 2. 初始化 VM
    this.vm.load(ast);

    // 3. 带监控执行
    try {
      const result = await this.vm.run(inputs);

      return {
        success: true,
        output: result,
        finalState: this.vm.captureState()
      };
    } catch (error) {
      return {
        success: false,
        error: this.runtime.handleError(error)
      };
    }
  }
}
```

#### 5.2 测试套件架构
```
test/
├── unit/
│   ├── lexer_test.ts
│   ├── parser_test.ts
│   ├── typechecker_test.ts
│   ├── constraints_test.ts
│   ├── runtime_test.ts
│   └── error_dso_test.ts
├── integration/
│   ├── full_pipeline_test.ts
│   ├── decompilation_test.ts
│   └── contract_verification_test.ts
├── fuzz/
│   ├── parser_fuzz.ts
│   └── constraint_fuzz.ts
├── benchmarks/
│   ├── token_efficiency/
│   ├── verification_speed/
│   └── binary_size/
└── samples/
    ├── valid/     # 正确的程序
    ├── invalid/   # 有错误的程序
    └── contracts/ # 契约违规示例
```

#### 5.3 性能基准测试
```typescript
// Token 效率
benchmark("token_efficiency", () => {
  const python = "def fib(n):\n  if n <= 1: return n\n  return fib(n-1) + fib(n-2)";
  const synton = "!fn:fib [n:i32]->i32 @pre(n>=0) (branch (<= n 1) n (+ (call:fib (- n 1)) (call:fib (- n 2))))";

  const pyTokens = countTokens(python);  // ~35
  const syTokens = countTokens(synton);  // ~22
  // 目标：syTokens < 0.7 * pyTokens
});

// 验证速度
benchmark("constraint_checking", () => {
  const constraints = generateTestConstraints(100);

  const start = Date.now();
  verifier.verifyConstraints(constraints);
  const duration = Date.now() - start;

  // 目标：100 个约束 <1000ms
});

// 二进制大小
benchmark("compression_ratio", () => {
  const source = loadSource("large_program.synton");
  const binary = serializer.serialize(parse(source));

  const ratio = binary.length / source.length;

  // 目标：ratio < 0.3
});
```

#### 5.4 测试覆盖目标
- 单元测试：95%+ 覆盖率
- 集成测试：所有主要工作流
- 基于属性的测试：所有纯函数
- 模糊测试：解析器和约束求解器
- 黄金主测试：反编译输出

---

### 阶段 6: 工具与文档（第 14-15 周）

#### 6.1 CLI 工具
```bash
# 编译 Synton 源码
synton build main.synton --output main.astb

# 执行二进制
synton run main.astb --input '{"data": [1,2,3]}'

# 仅验证契约
synton verify main.synton --strict

# 反编译为 Python
synton decompile main.astb --format python --output main.py

# 交互式 shell
synton repl
```

#### 6.2 语言服务器协议（LSP）
```typescript
class SyntonLanguageServer {
  // 提供 IDE 支持
  onHover(position): HoverInfo {
    // 显示位置处的类型和契约
  }

  onCompletion(position): CompletionItem[] {
    // 建议带契约的函数
  }

  onDiagnostic(file): Diagnostic[] {
    // 使用 DSO 进行实时错误检查
  }

  onDecompilationRequest(range): string {
    // 显示 Python/TypeScript 等效代码
  }
}
```

#### 6.3 文档
- **用户指南**：如何编写 Synton 代码
- **类型系统指南**：契约和精化类型
- **错误处理指南**：理解和使用 DSO
- **API 参考**：内置函数和类型
- **内部实现**：贡献者架构
- **形式化语义**：数学规范

---

## 依赖关系

### 外部依赖

| 组件 | 技术 | 版本 | 用途 |
|-----------|-----------|---------|---------|
| SMT 求解器 | Z3 | 4.12+ | 约束验证 |
| 解析器生成器 | 手写 | - | 完全控制，无依赖 |
| 运行时 | WebAssembly | - | 确定性执行 |
| 构建工具 | TypeScript | 5.0+ | 类型安全实现 |
| 测试 | Jest + 基于属性 | - | 全面测试 |

### 内部依赖

```
阶段 1 (AST/解析器)
    ↓
阶段 2 (类型系统) ← 阶段 3 (概率类型)
    ↓                      ↓
阶段 4 (错误协议) ←─┘
    ↓
阶段 5 (集成)
    ↓
阶段 6 (工具)
```

---

## 风险评估

### 关键风险

| 风险 | 影响 | 概率 | 缓解措施 |
|------|--------|-------------|------------|
| **SMT 求解器性能** | Z3 查询对生产来说太慢 | 高 | - 缓存约束结果<br>- 增量验证<br>- 超时回退到静态检查 |
| **歧义语法** | 解析器非确定性 | 中 | - LL(1) 属性的形式化证明<br>- 广泛的解析器测试<br>- 边缘情况模糊测试 |
| **契约表达力** | 无法表达重要约束 | 中 | - 支持类似 Python 的表达式<br>- 扩展自定义谓词<br>- 清楚记录限制 |
| **状态爆炸** | 复杂程序的 DSO 太大 | 中 | - 选择性状态捕获<br>- 大值压缩<br>- 可配置详细级别 |
| **反编译保真度** | 翻译中丢失语义 | 低 | - 形式化翻译语义<br>- 往返测试<br>- 关键路径手动审计 |

### 中等风险

| 风险 | 影响 | 概率 | 缓解措施 |
|------|--------|-------------|------------|
| **Token 计数回归** | 语法随时间变得冗长 | 中 | - 新功能的 token 预算<br>- 自动化基准测试<br>- 定期语法审查 |
| **置信度衰减** | Maybe 类型置信度丢失太快 | 中 | - 谨慎的传播公式<br>- 可配置衰减率<br>- 置信度"提升"操作 |
| **人类不可读** | 反编译产生神秘代码 | 低 | - 带格式化的美化打印<br>- 保留变量名<br>- 添加解释性注释 |

---

## 实施时间表

**总持续时间：** 15 周

### 甘特图概览

```
第 1-3 周：   阶段 1：AST 与解析器            ████████████
第 4-7 周：   阶段 2：类型系统与契约           ████████████████████
第 6-9 周：   阶段 3：概率类型                 ████████████████████
第 8-10 周：  阶段 4：错误协议                     ██████████████████
第 11-13 周： 阶段 5：集成与测试                     ████████████████████
第 14-15 周： 阶段 6：工具与文档                           ████████████
```

### 里程碑

- **M1（第 3 周）：** 解析器完成，可以解析所有示例代码
- **M2（第 7 周）：** 类型系统验证基本契约
- **M3（第 9 周）：** Maybe 类型和模糊操作工作
- **M4（第 10 周）：** 为所有失败模式生成结构化错误
- **M5（第 13 周）：** 完整管道工作，测试套件通过
- **M6（第 15 周）：** CLI 工具、LSP 和文档完成

---

## 成功指标

### 技术指标

| 指标 | 目标 | 测量方式 |
|--------|--------|-------------|
| **Token 效率** | 相比 Python 减少 >40% | 自动化基准测试套件 |
| **验证准确性** | >99.5% 约束检测 | 带已知违规的测试套件 |
| **验证速度** | 典型函数 <100ms | 性能基准测试 |
| **二进制压缩** | 源大小的 <30% | 压缩率测试 |
| **解析速度** | >1MB/s | 性能基准测试 |
| **测试覆盖** | >95% | 代码覆盖工具 |
| **反编译准确性** | 100% 语义保持 | 往返测试 |

### 质量指标

- **零关键 bug**（无数据丢失，无挂起）
- **错误恢复成功率** 常见问题 >70%
- **用户满意度**（基于试点计划）>4/5
- **文档完整性** - 所有 API 已文档化

---

## 待解决问题与决策点

### 开始前需要解决

1. **契约语法**：使用 `@pre/@post` 还是 `where` 子句？
   - **建议**：`@pre/@post` 注解（更清晰的分离）

2. **置信度表示**：定点还是浮点？
   - **建议**：浮点（0.0-1.0）以获得灵活性

3. **SMT 求解器超时**：放弃前等多久？
   - **建议**：默认 1000ms，可配置

4. **DSO 格式**：JSON 还是二进制（MessagePack/Protobuf）？
   - **建议**：JSON 用于人工调试，MessagePack 用于生产

### 实施期间决定

1. **标准库范围**：包含哪些内置函数？
2. **张量操作集**：原生支持哪些操作？
3. **错误恢复策略**：自动修复还是手动干预？
4. **反编译美化**：可读性与保真度之间的权衡？

---

## 下一步

### 批准后的立即行动

1. **第 1 周启动**
   - 设置开发环境（monorepo、CI/CD）
   - 为阶段 1 创建详细任务分解
   - 设置项目管理（里程碑、依赖关系）
   - 入职团队成员

2. **基础设施设置**
   - 仓库结构
   - 构建系统（TypeScript + Node.js）
   - 测试框架（Jest + fast-check 用于属性测试）
   - CI/CD 管道（GitHub Actions）
   - 文档站点（VitePress）

3. **开发环境**
   - VS Code 工作区带扩展
   - 格式化器（Prettier）和 linter（ESLint）
   - 预提交钩子（Husky）
   - 本地 Z3 集成

### 第一个开发冲刺（第 1-2 周）

- 定义语法 EBNF
- 实现基本词法分析器
- 启动带表达式处理的解析器
- 创建前 10 个测试样本

---

## 附录

### A. 示例程序

**A.1 Hello World**
```synton
!fn:main [] -> i32
  (call:print "Hello, World!")
  0
```

**A.2 带契约的阶乘**
```synton
!fn:fact [n:i32] -> i32
  @pre(n >= 0)
  @post($ret >= 1 || $ret == 0)
  (branch (== n 0)
    1
    (* n (call:fact (- n 1)))
  )
```

**A.3 概率字符串匹配**
```synton
!fn:extract_name [text:string] -> maybe<string, dynamic>
  (def patterns ["Mr. ", "Ms. ", "Dr. "])
  (def result
    (fuzzy_find_any patterns text)
  )
  (branch (result.is_certain 0.85)
    result.value
    (maybe:nothing)
  )
```

### B. 错误 DSO 示例

**B.1 类型不匹配**
```json
{
  "status": "CompileError",
  "error_code": "TYPE_MISMATCH",
  "location": {
    "node_id": "node_45a",
    "source_span": {"file": "fib.synton", "start_line": 5, "start_col": 10}
  },
  "error_details": {
    "expected": "i32",
    "actual": "string",
    "reason": "Cannot add string to i32"
  },
  "suggestions": [{
    "type": "fix",
    "description": "Convert string to i32 before addition",
    "code": "(+ (cast x i32) y)",
    "confidence": 0.95
  }]
}
```

**B.2 契约违规**
```json
{
  "status": "ConstraintViolation",
  "error_code": "POSTCONDITION_FAILED",
  "location": {
    "node_id": "node_78b"
  },
  "context": {
    "variable_values": {"n": -1, "$ret": -2},
    "contract_states": [{
      "contract": "@post($ret >= n)",
      "is_satisfied": false,
      "counterexample": {"n": -1, "$ret": -2}
    }]
  },
  "error_details": {
    "expected": "$ret >= n",
    "actual": "-2 < -1",
    "reason": "Postcondition violated"
  },
  "suggestions": [{
    "type": "fix",
    "description": "Strengthen precondition",
    "code": "@pre(n >= 0)",
    "confidence": 0.90
  }]
}
```

### C. 参考文献

- **Z3 定理证明器**：https://github.com/Z3Prover/z3
- **精化类型**：Liquid Haskell、Dafny
- **Maybe/Monad 类型**：Haskell、Rust
- **S 表达式**：Lisp、Scheme
- **WebAssembly**：https://webassembly.org/

---

**文档版本：** 1.0
**最后更新：** 2025-02-01
**状态：** 🟡 等待批准
