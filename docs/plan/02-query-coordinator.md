# 查询协调器模块开发计划

## 一、模块概述

查询协调器是中间层的核心组件，负责接收来自 API 网关的查询请求，根据查询类型路由到不同的搜索引擎，并协调多个搜索引擎的并行执行和结果融合。

---

## 二、功能需求

### 2.1 核心功能

| 功能 | 描述 | 优先级 |
|------|------|--------|
| **查询路由** | 根据查询类型和配置路由到合适的搜索引擎 | P0 |
| **并行执行** | 并行调用多个搜索引擎 | P0 |
| **结果融合** | 融合多个搜索引擎的结果 | P0 |
| **超时控制** | 控制每个查询的超时时间 | P0 |
| **错误处理** | 处理搜索引擎的错误和故障 | P0 |
| **缓存** | 缓存查询结果 | P1 |
| **查询优化** | 优化查询以提高性能 | P1 |
| **监控指标** | 收集查询性能指标 | P2 |

### 2.2 查询路由策略

| 查询类型 | 路由策略 | 理由 |
|---------|---------|------|
| **精确匹配** | FlexSearch | 性能最优 |
| **短语搜索** | FlexSearch | 支持短语匹配 |
| **模糊搜索** | FlexSearch | 支持模糊匹配 |
| **语义搜索** | Vector Search | 语义理解 |
| **混合搜索** | Hybrid (FlexSearch + Vector) | 兼顾精确和语义 |
| **复杂查询** | BM25 | 支持复杂评分 |
| **自动选择** | Coordinator 根据查询特征自动选择 | 智能路由 |

---

## 三、技术选型

### 3.1 语言和框架

| 技术 | 版本 | 用途 |
|------|------|------|
| **TypeScript** | 5.x | 类型安全 |
| **Node.js** | 20.x | 运行时 |
| **gRPC** | 1.x | 服务间通信 |

### 3.2 依赖库

```json
{
  "dependencies": {
    "@grpc/grpc-js": "^1.10.0",
    "@grpc/proto-loader": "^0.7.10",
    "redis": "^4.7.0",
    "flexsearch": "^0.8.200",
    "axios": "^1.6.5",
    "pino": "^8.19.0",
    "p-queue": "^8.0.1",
    "p-timeout": "^6.1.2",
    "lodash": "^4.17.21",
    "dotenv": "^16.3.1"
  },
  "devDependencies": {
    "@types/node": "^20.11.0",
    "@types/lodash": "^4.14.202",
    "typescript": "^5.3.3",
    "jest": "^29.7.0",
    "@types/jest": "^29.5.11"
  }
}
```

---

## 四、架构设计

### 4.1 整体架构

```
┌─────────────────────────────────────────┐
│         API 网关               │
└──────────────┬──────────────────────┘
               │ gRPC
               ▼
┌─────────────────────────────────────────┐
│         查询协调器               │
│  ┌──────────────┬──────────────┐       │
│  │ 查询路由器   │ 结果融合器   │       │
│  └──────────────┴──────────────┘       │
│  ┌──────────────┬──────────────┐       │
│  │ 查询优化器   │ 缓存管理器   │       │
│  └──────────────┴──────────────┘       │
└──────────────┬──────────────────────┘
               │
        ┌──────┼──────┬──────────┐
        │      │        │          │
        ▼      ▼        ▼          ▼
┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐
│FlexSearch│ │  BM25    │ │  向量搜索  │ │  缓存    │
│ 服务     │ │  服务     │ │  服务      │ │  服务    │
└──────────┘ └──────────┘ └──────────┘ └──────────┘
```

### 4.2 目录结构

```
services/coordinator/
├── src/
│   ├── index.ts                 # 入口文件
│   ├── server.ts               # gRPC 服务器
│   ├── config/                 # 配置
│   │   ├── index.ts
│   │   └── engines.ts
│   ├── router/                 # 查询路由器
│   │   ├── index.ts
│   │   ├── strategy.ts
│   │   └── optimizer.ts
│   ├── merger/                 # 结果融合器
│   │   ├── index.ts
│   │   ├── rrf.ts
│   │   ├── weighted.ts
│   │   └── learning.ts
│   ├── engines/                # 搜索引擎客户端
│   │   ├── base.ts
│   │   ├── flexsearch.ts
│   │   ├── bm25.ts
│   │   └── vector.ts
│   ├── cache/                  # 缓存管理
│   │   ├── index.ts
│   │   └── redis.ts
│   ├── types/                  # 类型定义
│   │   └── index.ts
│   └── utils/                  # 工具函数
│       ├── logger.ts
│       └── metrics.ts
├── proto/
│   └── coordinator.proto       # gRPC 协议定义
├── tests/
│   ├── unit/
│   └── integration/
├── package.json
├── tsconfig.json
├── Dockerfile
└── README.md
```

---

## 五、核心实现

### 5.1 gRPC 协议定义

```protobuf
syntax = "proto3";

package coordinator;

service QueryCoordinator {
  rpc Search(SearchRequest) returns (SearchResponse);
  rpc GetStatus(StatusRequest) returns (StatusResponse);
}

message SearchRequest {
  string query = 1;
  int32 limit = 2;
  int32 offset = 3;
  map<string, string> filters = 4;
  SearchOptions options = 5;
}

message SearchOptions {
  string engine = 1;  // auto, flexsearch, bm25, vector, hybrid
  bool enrich = 2;
  bool highlight = 3;
  float alpha = 4;  // 关键词权重
  float beta = 5;   // 向量权重
}

message SearchResponse {
  repeated SearchResult results = 1;
  int32 total = 2;
  int32 latency = 3;
  string engine = 4;
  map<string, string> metadata = 5;
}

message SearchResult {
  string id = 1;
  float score = 2;
  Document doc = 3;
  repeated string highlights = 4;
}

message Document {
  string title = 1;
  string content = 2;
  map<string, string> metadata = 3;
}

message StatusRequest {}

message StatusResponse {
  bool healthy = 1;
  map<string, EngineStatus> engines = 2;
}

message EngineStatus {
  bool available = 1;
  int32 latency = 2;
  int32 error_rate = 3;
}
```

### 5.2 查询协调器

```typescript
import { QueryRouter } from './router';
import { ResultMerger } from './merger';
import { CacheManager } from './cache';
import { Logger } from './utils/logger';
import { Metrics } from './utils/metrics';

export class QueryCoordinator {
  private router: QueryRouter;
  private merger: ResultMerger;
  private cache: CacheManager;
  private logger: Logger;
  private metrics: Metrics;

  constructor() {
    this.router = new QueryRouter();
    this.merger = new ResultMerger();
    this.cache = new CacheManager();
    this.logger = new Logger('coordinator');
    this.metrics = new Metrics();
  }

  async search(request: SearchRequest): Promise<SearchResponse> {
    const startTime = Date.now();
    const cacheKey = this.getCacheKey(request);

    try {
      // 1. 检查缓存
      const cached = await this.cache.get(cacheKey);
      if (cached) {
        this.logger.info('Cache hit', { query: request.query });
        this.metrics.increment('cache_hit');
        return cached;
      }

      // 2. 路由查询
      const engines = await this.router.route(request);
      this.logger.info('Routing query', {
        query: request.query,
        engines: engines.map(e => e.name)
      });

      // 3. 并行执行查询
      const results = await this.executeQueries(engines, request);

      // 4. 融合结果
      const merged = await this.merger.merge(results, request);

      // 5. 构建响应
      const response: SearchResponse = {
        results: merged.items,
        total: merged.total,
        latency: Date.now() - startTime,
        engine: merged.engine,
        metadata: merged.metadata
      };

      // 6. 缓存结果
      await this.cache.set(cacheKey, response, 300);  // 5 分钟

      // 7. 记录指标
      this.metrics.histogram('search_latency', response.latency);
      this.metrics.increment('search_total');

      return response;
    } catch (error) {
      this.logger.error('Search failed', { error, request });
      this.metrics.increment('search_error');
      throw error;
    }
  }

  private async executeQueries(
    engines: SearchEngine[],
    request: SearchRequest
  ): Promise<EngineResult[]> {
    const timeout = request.options.timeout || 5000;  // 5 秒超时

    const promises = engines.map(engine =>
      pTimeout(engine.search(request), timeout)
        .catch(error => {
          this.logger.warn('Engine search failed', {
            engine: engine.name,
            error
          });
          return null;
        })
    );

    const results = await Promise.all(promises);
    return results.filter(r => r !== null) as EngineResult[];
  }

  private getCacheKey(request: SearchRequest): string {
    const hash = crypto
      .createHash('md5')
      .update(JSON.stringify(request))
      .digest('hex');
    return `search:${hash}`;
  }
}
```

### 5.3 查询路由器

```typescript
import { SearchEngine } from '../engines/base';
import { FlexSearchEngine } from '../engines/flexsearch';
import { BM25Engine } from '../engines/bm25';
import { VectorEngine } from '../engines/vector';

export class QueryRouter {
  private engines: Map<string, SearchEngine>;

  constructor() {
    this.engines = new Map();
    this.initializeEngines();
  }

  private async initializeEngines() {
    this.engines.set('flexsearch', new FlexSearchEngine());
    this.engines.set('bm25', new BM25Engine());
    this.engines.set('vector', new VectorEngine());
  }

  async route(request: SearchRequest): Promise<SearchEngine[]> {
    const { engine, options } = request;

    // 显式指定引擎
    if (engine && engine !== 'auto') {
      const targetEngine = this.engines.get(engine);
      if (targetEngine) {
        return [targetEngine];
      }
    }

    // 自动路由
    return this.autoRoute(request);
  }

  private autoRoute(request: SearchRequest): SearchEngine[] {
    const { query, options } = request;

    // 短查询（< 3 个词）：使用 FlexSearch
    if (query.split(/\s+/).length < 3) {
      return [this.engines.get('flexsearch')!];
    }

    // 包含特殊字符：使用 FlexSearch（模糊搜索）
    if (/[?*~]/.test(query)) {
      return [this.engines.get('flexsearch')!];
    }

    // 启用语义搜索：使用向量搜索
    if (options?.semantic) {
      return [this.engines.get('vector')!];
    }

    // 默认：混合搜索
    return [
      this.engines.get('flexsearch')!,
      this.engines.get('vector')!
    ];
  }
}
```

### 5.4 结果融合器

```typescript
export class ResultMerger {
  private strategies: Map<string, MergeStrategy>;

  constructor() {
    this.strategies = new Map();
    this.initializeStrategies();
  }

  private initializeStrategies() {
    this.strategies.set('rrf', new RRFStrategy());
    this.strategies.set('weighted', new WeightedStrategy());
    this.strategies.set('learning', new LearningStrategy());
  }

  async merge(
    results: EngineResult[],
    request: SearchRequest
  ): Promise<MergedResult> {
    if (results.length === 0) {
      return { items: [], total: 0, engine: 'none', metadata: {} };
    }

    if (results.length === 1) {
      return {
        items: results[0].results,
        total: results[0].total,
        engine: results[0].engine,
        metadata: results[0].metadata || {}
      };
    }

    // 选择融合策略
    const strategy = this.selectStrategy(request);
    return strategy.merge(results, request);
  }

  private selectStrategy(request: SearchRequest): MergeStrategy {
    const { options } = request;

    if (options?.mergeStrategy) {
      return this.strategies.get(options.mergeStrategy)!;
    }

    // 默认使用 RRF
    return this.strategies.get('rrf')!;
  }
}

// RRF（Reciprocal Rank Fusion）策略
class RRFStrategy implements MergeStrategy {
  merge(results: EngineResult[], request: SearchRequest): MergedResult {
    const k = 60;  // RRF 常数
    const scores = new Map<string, number>();

    results.forEach(result => {
      result.results.forEach((item, index) => {
        const id = item.id;
        const score = scores.get(id) || 0;
        scores.set(id, score + 1 / (k + index + 1));
      });
    });

    const sorted = Array.from(scores.entries())
      .sort((a, b) => b[1] - a[1])
      .slice(0, request.limit)
      .map(([id, score]) => ({ id, score }));

    return {
      items: sorted,
      total: sorted.length,
      engine: 'hybrid',
      metadata: { strategy: 'rrf' }
    };
  }
}

// 加权融合策略
class WeightedStrategy implements MergeStrategy {
  merge(results: EngineResult[], request: SearchRequest): MergedResult {
    const { options } = request;
    const alpha = options?.alpha || 0.7;  // FlexSearch 权重
    const beta = options?.beta || 0.3;     // 向量搜索权重
    const scores = new Map<string, number>();

    results.forEach(result => {
      const weight = result.engine === 'flexsearch' ? alpha : beta;
      result.results.forEach(item => {
        const id = item.id;
        const score = scores.get(id) || 0;
        scores.set(id, score + item.score * weight);
      });
    });

    const sorted = Array.from(scores.entries())
      .sort((a, b) => b[1] - a[1])
      .slice(0, request.limit)
      .map(([id, score]) => ({ id, score }));

    return {
      items: sorted,
      total: sorted.length,
      engine: 'hybrid',
      metadata: { strategy: 'weighted', alpha, beta }
    };
  }
}
```

### 5.5 搜索引擎客户端

```typescript
import { SearchEngine, EngineResult } from './base';

export class FlexSearchEngine implements SearchEngine {
  name = 'flexsearch';
  type = EngineType.KEYWORD;
  private index: any;

  async initialize(config: EngineConfig) {
    const { Index } = await import('flexsearch');
    this.index = new Index({
      tokenize: 'strict',
      resolution: 9,
      optimize: true,
      cache: true
    });
  }

  async search(request: SearchRequest): Promise<EngineResult> {
    const startTime = Date.now();
    const results = this.index.search(request.query, request.limit);

    return {
      engine: this.name,
      results: results.map(id => ({ id, score: 1 })),
      total: results.length,
      latency: Date.now() - startTime,
      metadata: {}
    };
  }

  async addDocument(doc: Document): Promise<void> {
    this.index.add(doc.id, doc.content);
  }

  async removeDocument(id: string): Promise<void> {
    this.index.remove(id);
  }
}

export class BM25Engine implements SearchEngine {
  name = 'bm25';
  type = EngineType.KEYWORD;
  private client: any;

  async initialize(config: EngineConfig) {
    this.client = await import('flexsearch/db/postgres');
    await this.client.open(config);
  }

  async search(request: SearchRequest): Promise<EngineResult> {
    const startTime = Date.now();
    const results = await this.client.search(request.query, request.limit);

    return {
      engine: this.name,
      results: results.map(r => ({ id: r.id, score: r.score })),
      total: results.length,
      latency: Date.now() - startTime,
      metadata: {}
    };
  }
}

export class VectorEngine implements SearchEngine {
  name = 'vector';
  type = EngineType.VECTOR;
  private client: any;

  async initialize(config: EngineConfig) {
    const { QdrantClient } = await import('@qdrant/js-client-rest');
    this.client = new QdrantClient({ url: config.url });
  }

  async search(request: SearchRequest): Promise<EngineResult> {
    const startTime = Date.now();

    // 生成查询向量
    const queryVector = await this.generateEmbedding(request.query);

    // 搜索
    const results = await this.client.search(request.collection || 'default', {
      vector: queryVector,
      limit: request.limit,
      with_payload: true
    });

    return {
      engine: this.name,
      results: results.map(r => ({ id: r.id, score: r.score })),
      total: results.length,
      latency: Date.now() - startTime,
      metadata: {}
    };
  }

  private async generateEmbedding(text: string): Promise<number[]> {
    // 调用嵌入模型生成向量
    // 这里可以集成各种嵌入模型（如 sentence-transformers）
    return [];
  }
}
```

---

## 六、开发计划

### 6.1 任务分解

| 任务 | 预估时间 | 优先级 | 依赖 |
|------|---------|--------|------|
| **项目初始化** | 1 天 | P0 | - |
| - 创建项目结构 | 0.5 天 | P0 | - |
| - 配置 TypeScript | 0.5 天 | P0 | - |
| **gRPC 服务** | 3 天 | P0 | 项目初始化 |
| - 定义协议 | 0.5 天 | P0 | - |
| - 实现服务器 | 1 天 | P0 | - |
| - 实现客户端 | 1 天 | P0 | - |
| - 健康检查 | 0.5 天 | P0 | - |
| **查询路由器** | 3 天 | P0 | gRPC 服务 |
| - 路由策略 | 1 天 | P0 | - |
| - 查询优化 | 1 天 | P0 | - |
| - 自动路由 | 1 天 | P0 | - |
| **结果融合器** | 4 天 | P0 | 查询路由器 |
| - RRF 策略 | 1 天 | P0 | - |
| - 加权策略 | 1 天 | P0 | - |
| - 学习策略 | 2 天 | P1 | - |
| **搜索引擎客户端** | 4 天 | P0 | 结果融合器 |
| - FlexSearch 客户端 | 1 天 | P0 | - |
| - BM25 客户端 | 1.5 天 | P0 | - |
| - 向量搜索客户端 | 1.5 天 | P0 | - |
| **缓存管理** | 2 天 | P1 | 搜索引擎客户端 |
| - Redis 缓存 | 1 天 | P1 | - |
| - 缓存策略 | 1 天 | P1 | - |
| **监控指标** | 2 天 | P2 | 缓存管理 |
| - 指标收集 | 1 天 | P2 | - |
| - 指标导出 | 1 天 | P2 | - |
| **测试** | 4 天 | P1 | 所有功能 |
| - 单元测试 | 2 天 | P1 | - |
| - 集成测试 | 1 天 | P1 | - |
| - 压力测试 | 1 天 | P1 | - |
| **Docker 化** | 1 天 | P2 | 测试 |
| - Dockerfile | 0.5 天 | P2 | - |
| - docker-compose | 0.5 天 | P2 | - |
| **总计** | **26 天** | - | - |

### 6.2 里程碑

| 里程碑 | 交付物 | 时间 |
|--------|--------|------|
| **M1: gRPC 服务** | gRPC 服务器、客户端、协议定义 | 第 4 天 |
| **M2: 查询路由器** | 路由策略、查询优化、自动路由 | 第 7 天 |
| **M3: 结果融合器** | RRF、加权、学习融合策略 | 第 11 天 |
| **M4: 搜索引擎客户端** | FlexSearch、BM25、向量搜索客户端 | 第 15 天 |
| **M5: 缓存和监控** | Redis 缓存、指标收集和导出 | 第 18 天 |
| **M6: 测试完成** | 单元测试、集成测试、压力测试 | 第 22 天 |
| **M7: 部署就绪** | Docker 镜像、文档 | 第 23 天 |

---

## 七、测试策略

### 7.1 单元测试

```typescript
import { QueryCoordinator } from '../src/coordinator';

describe('QueryCoordinator', () => {
  let coordinator: QueryCoordinator;

  beforeEach(() => {
    coordinator = new QueryCoordinator();
  });

  test('should route query to flexsearch for short queries', async () => {
    const request = {
      query: 'test',
      limit: 10,
      offset: 0,
      options: {}
    };

    const response = await coordinator.search(request);
    expect(response.engine).toBe('flexsearch');
  });

  test('should merge results from multiple engines', async () => {
    const request = {
      query: 'test query',
      limit: 10,
      offset: 0,
      options: { engine: 'hybrid' }
    };

    const response = await coordinator.search(request);
    expect(response.results).toBeInstanceOf(Array);
    expect(response.engine).toBe('hybrid');
  });
});
```

### 7.2 集成测试

```typescript
describe('Integration Tests', () => {
  test('end-to-end search with caching', async () => {
    const request = {
      query: 'flexsearch',
      limit: 10,
      offset: 0,
      options: {}
    };

    // 第一次查询
    const response1 = await coordinator.search(request);
    expect(response1.results.length).toBeGreaterThan(0);

    // 第二次查询（应该命中缓存）
    const response2 = await coordinator.search(request);
    expect(response2.results).toEqual(response1.results);
  });
});
```

---

## 八、部署方案

### 8.1 Dockerfile

```dockerfile
FROM node:20-alpine AS builder

WORKDIR /app

COPY package*.json ./
RUN npm ci --only=production

COPY . .
RUN npm run build

FROM node:20-alpine

WORKDIR /app

COPY --from=builder /app/dist ./dist
COPY --from=builder /app/node_modules ./node_modules
COPY --from=builder /app/package.json ./

EXPOSE 8081

CMD ["node", "dist/index.js"]
```

### 8.2 Docker Compose

```yaml
version: '3.8'

services:
  coordinator:
    build: ./services/coordinator
    ports:
      - "8081:8081"
    environment:
      - NODE_ENV=production
      - FLEXSEARCH_URL=http://flexsearch:8083
      - BM25_URL=http://bm25:8084
      - VECTOR_URL=http://vector:8085
      - REDIS_URL=redis://redis:6379
      - LOG_LEVEL=info
    depends_on:
      - flexsearch
      - bm25
      - vector
      - redis
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "wget", "--quiet", "--tries=1", "--spider", "http://localhost:8081/health"]
      interval: 30s
      timeout: 10s
      retries: 3
```

---

## 九、监控和日志

### 9.1 监控指标

| 指标 | 类型 | 说明 |
|------|------|------|
| **search_total** | Counter | 搜索总数 |
| **search_latency** | Histogram | 搜索延迟 |
| **search_error** | Counter | 搜索错误数 |
| **cache_hit** | Counter | 缓存命中数 |
| **cache_miss** | Counter | 缓存未命中数 |
| **engine_latency** | Histogram | 各引擎延迟 |

### 9.2 日志格式

```json
{
  "level": "info",
  "time": "2024-01-01T00:00:00.000Z",
  "pid": 12345,
  "hostname": "coordinator-1",
  "component": "coordinator",
  "query": "test",
  "engines": ["flexsearch", "vector"],
  "latency": 50,
  "cache": "hit"
}
```

---

## 十、风险和缓解

| 风险 | 概率 | 影响 | 缓解措施 |
|------|------|------|---------|
| 查询延迟高 | 中 | 高 | 并行执行、超时控制 |
| 引擎故障 | 中 | 高 | 熔断机制、降级策略 |
| 结果融合质量 | 低 | 中 | 多种融合策略、A/B 测试 |
| 缓存一致性 | 低 | 中 | 合理的缓存策略 |

---

**文档版本**：1.0
**最后更新**：2026-02-04
**作者**：FlexSearch 技术分析团队
