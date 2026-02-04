# FlexSearch 服务模块开发计划

## 一、模块概述

FlexSearch 服务是中间层的核心搜索引擎之一，负责提供极致性能的关键词搜索功能。该服务直接复用 FlexSearch 核心引擎，通过 gRPC 接口对外提供服务。

---

## 二、功能需求

### 2.1 核心功能

| 功能 | 描述 | 优先级 |
|------|------|--------|
| **文档索引** | 添加、更新、删除文档 | P0 |
| **关键词搜索** | 精确匹配、模糊搜索、短语搜索 | P0 |
| **缓存管理** | 自动缓存热门查询 | P0 |
| **持久化** | 支持多种持久化后端 | P0 |
| **批量操作** | 支持批量添加和删除 | P1 |
| **高亮显示** | 搜索结果高亮 | P1 |
| **建议功能** | 搜索建议和自动完成 | P2 |
| **监控指标** | 收集性能指标 | P2 |

### 2.2 搜索特性

| 特性 | FlexSearch 支持 | 说明 |
|------|----------------|------|
| **精确匹配** | ✅ | 完全匹配查询词 |
| **模糊匹配** | ✅ | 支持通配符（*、?） |
| **短语搜索** | ✅ | 支持引号短语 |
| **布尔查询** | ✅ | AND、OR、NOT |
| **范围查询** | ✅ | 数值、日期范围 |
| **前缀搜索** | ✅ | 支持前缀匹配 |
| **后缀搜索** | ✅ | 支持后缀匹配 |

---

## 三、技术选型

### 3.1 核心依赖

| 依赖 | 版本 | 用途 |
|------|------|------|
| **flexsearch** | 0.8.200 | 核心搜索引擎 |
| **@grpc/grpc-js** | 1.10.0 | gRPC 服务 |
| **redis** | 4.7.0 | 缓存和持久化 |
| **pino** | 8.19.0 | 日志系统 |

### 3.2 依赖库

```json
{
  "dependencies": {
    "flexsearch": "^0.8.200",
    "@grpc/grpc-js": "^1.10.0",
    "@grpc/proto-loader": "^0.7.10",
    "redis": "^4.7.0",
    "pino": "^8.19.0",
    "pino-pretty": "^10.3.1",
    "dotenv": "^16.3.1",
    "lodash": "^4.17.21"
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
│         查询协调器               │
└──────────────┬──────────────────────┘
               │ gRPC
               ▼
┌─────────────────────────────────────────┐
│         FlexSearch 服务         │
│  ┌──────────────┬──────────────┐       │
│  │ 索引管理器   │ 搜索管理器   │       │
│  └──────────────┴──────────────┘       │
│  ┌──────────────┬──────────────┐       │
│  │ 缓存管理器   │ 持久化管理器 │       │
│  └──────────────┴──────────────┘       │
└──────────────┬──────────────────────┘
               │
        ┌──────┼──────┐
        │      │        │
        ▼      ▼        ▼
┌──────────┐ ┌──────────┐ ┌──────────┐
│FlexSearch│ │  Redis   │ │  文档存储 │
│ 核心     │ │  缓存    │ │  (可选)  │
└──────────┘ └──────────┘ └──────────┘
```

### 4.2 目录结构

```
services/flexsearch/
├── src/
│   ├── index.ts                 # 入口文件
│   ├── server.ts               # gRPC 服务器
│   ├── config/                 # 配置
│   │   ├── index.ts
│   │   └── flexsearch.ts
│   ├── index/                  # 索引管理
│   │   ├── manager.ts
│   │   ├── builder.ts
│   │   └── optimizer.ts
│   ├── search/                 # 搜索管理
│   │   ├── manager.ts
│   │   ├── highlighter.ts
│   │   └── suggester.ts
│   ├── cache/                  # 缓存管理
│   │   ├── index.ts
│   │   └── redis.ts
│   ├── storage/                # 持久化
│   │   ├── index.ts
│   │   └── redis.ts
│   ├── types/                  # 类型定义
│   │   └── index.ts
│   └── utils/                  # 工具函数
│       ├── logger.ts
│       └── metrics.ts
├── proto/
│   └── flexsearch.proto       # gRPC 协议定义
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

package flexsearch;

service FlexSearchService {
  rpc AddDocument(AddDocumentRequest) returns (AddDocumentResponse);
  rpc BatchAddDocuments(BatchAddDocumentsRequest) returns (BatchAddDocumentsResponse);
  rpc UpdateDocument(UpdateDocumentRequest) returns (UpdateDocumentResponse);
  rpc RemoveDocument(RemoveDocumentRequest) returns (RemoveDocumentResponse);
  rpc Search(SearchRequest) returns (SearchResponse);
  rpc Suggest(SuggestRequest) returns (SuggestResponse);
  rpc GetStatus(StatusRequest) returns (StatusResponse);
}

message AddDocumentRequest {
  string id = 1;
  string content = 2;
  map<string, string> metadata = 3;
}

message AddDocumentResponse {
  bool success = 1;
  string message = 2;
}

message BatchAddDocumentsRequest {
  repeated Document documents = 1;
}

message BatchAddDocumentsResponse {
  int32 added = 1;
  int32 failed = 2;
  repeated string errors = 3;
}

message UpdateDocumentRequest {
  string id = 1;
  string content = 2;
  map<string, string> metadata = 3;
}

message UpdateDocumentResponse {
  bool success = 1;
  string message = 2;
}

message RemoveDocumentRequest {
  string id = 1;
}

message RemoveDocumentResponse {
  bool success = 1;
  string message = 2;
}

message SearchRequest {
  string query = 1;
  int32 limit = 2;
  int32 offset = 3;
  map<string, string> filters = 4;
  SearchOptions options = 5;
}

message SearchOptions {
  bool enrich = 1;
  bool highlight = 2;
  bool suggest = 3;
  string resolution = 4;  // basic, advanced
}

message SearchResponse {
  repeated SearchResult results = 1;
  int32 total = 2;
  int32 latency = 3;
  map<string, string> metadata = 4;
}

message SearchResult {
  string id = 1;
  float score = 2;
  Document doc = 3;
  repeated string highlights = 4;
}

message Document {
  string id = 1;
  string content = 2;
  map<string, string> metadata = 3;
}

message SuggestRequest {
  string query = 1;
  int32 limit = 2;
}

message SuggestResponse {
  repeated string suggestions = 1;
}

message StatusRequest {}

message StatusResponse {
  bool healthy = 1;
  int64 document_count = 2;
  int64 index_size = 3;
  map<string, string> metadata = 4;
}
```

### 5.2 FlexSearch 服务

```typescript
import { IndexManager } from './index/manager';
import { SearchManager } from './search/manager';
import { CacheManager } from './cache';
import { StorageManager } from './storage';
import { Logger } from './utils/logger';
import { Metrics } from './utils/metrics';

export class FlexSearchService {
  private indexManager: IndexManager;
  private searchManager: SearchManager;
  private cacheManager: CacheManager;
  private storageManager: StorageManager;
  private logger: Logger;
  private metrics: Metrics;

  constructor() {
    this.indexManager = new IndexManager();
    this.searchManager = new SearchManager(this.indexManager);
    this.cacheManager = new CacheManager();
    this.storageManager = new StorageManager();
    this.logger = new Logger('flexsearch');
    this.metrics = new Metrics();
  }

  async initialize(): Promise<void> {
    await this.indexManager.initialize();
    await this.cacheManager.initialize();
    await this.storageManager.initialize();
    this.logger.info('FlexSearch service initialized');
  }

  async addDocument(request: AddDocumentRequest): Promise<AddDocumentResponse> {
    const startTime = Date.now();

    try {
      await this.indexManager.addDocument({
        id: request.id,
        content: request.content,
        metadata: request.metadata
      });

      this.metrics.increment('document_added');
      this.metrics.histogram('add_document_latency', Date.now() - startTime);

      return { success: true, message: 'Document added successfully' };
    } catch (error) {
      this.logger.error('Failed to add document', { error, request });
      this.metrics.increment('document_add_error');
      return { success: false, message: error.message };
    }
  }

  async batchAddDocuments(
    request: BatchAddDocumentsRequest
  ): Promise<BatchAddDocumentsResponse> {
    const startTime = Date.now();
    let added = 0;
    let failed = 0;
    const errors: string[] = [];

    for (const doc of request.documents) {
      try {
        await this.indexManager.addDocument(doc);
        added++;
      } catch (error) {
        failed++;
        errors.push(`${doc.id}: ${error.message}`);
      }
    }

    this.metrics.increment('documents_added', added);
    this.metrics.increment('documents_failed', failed);
    this.metrics.histogram('batch_add_latency', Date.now() - startTime);

    return { added, failed, errors };
  }

  async updateDocument(
    request: UpdateDocumentRequest
  ): Promise<UpdateDocumentResponse> {
    const startTime = Date.now();

    try {
      await this.indexManager.updateDocument({
        id: request.id,
        content: request.content,
        metadata: request.metadata
      });

      this.metrics.increment('document_updated');
      this.metrics.histogram('update_document_latency', Date.now() - startTime);

      return { success: true, message: 'Document updated successfully' };
    } catch (error) {
      this.logger.error('Failed to update document', { error, request });
      this.metrics.increment('document_update_error');
      return { success: false, message: error.message };
    }
  }

  async removeDocument(
    request: RemoveDocumentRequest
  ): Promise<RemoveDocumentResponse> {
    const startTime = Date.now();

    try {
      await this.indexManager.removeDocument(request.id);

      this.metrics.increment('document_removed');
      this.metrics.histogram('remove_document_latency', Date.now() - startTime);

      return { success: true, message: 'Document removed successfully' };
    } catch (error) {
      this.logger.error('Failed to remove document', { error, request });
      this.metrics.increment('document_remove_error');
      return { success: false, message: error.message };
    }
  }

  async search(request: SearchRequest): Promise<SearchResponse> {
    const startTime = Date.now();
    const cacheKey = this.getCacheKey(request);

    try {
      // 1. 检查缓存
      const cached = await this.cacheManager.get(cacheKey);
      if (cached) {
        this.logger.info('Cache hit', { query: request.query });
        this.metrics.increment('cache_hit');
        return cached;
      }

      // 2. 执行搜索
      const results = await this.searchManager.search(request);

      // 3. 高亮显示
      if (request.options?.highlight) {
        results.results = this.searchManager.highlight(results.results, request.query);
      }

      // 4. 构建响应
      const response: SearchResponse = {
        results: results.items,
        total: results.total,
        latency: Date.now() - startTime,
        metadata: {
          engine: 'flexsearch',
          cached: false
        }
      };

      // 5. 缓存结果
      await this.cacheManager.set(cacheKey, response, 300);

      // 6. 记录指标
      this.metrics.histogram('search_latency', response.latency);
      this.metrics.increment('search_total');

      return response;
    } catch (error) {
      this.logger.error('Search failed', { error, request });
      this.metrics.increment('search_error');
      throw error;
    }
  }

  async suggest(request: SuggestRequest): Promise<SuggestResponse> {
    const suggestions = await this.searchManager.suggest(request.query, request.limit);
    return { suggestions };
  }

  async getStatus(): Promise<StatusResponse> {
    const stats = await this.indexManager.getStats();
    return {
      healthy: true,
      document_count: stats.documentCount,
      index_size: stats.indexSize,
      metadata: {
        version: '0.8.200',
        uptime: process.uptime().toString()
      }
    };
  }

  private getCacheKey(request: SearchRequest): string {
    const hash = crypto
      .createHash('md5')
      .update(JSON.stringify(request))
      .digest('hex');
    return `flexsearch:${hash}`;
  }
}
```

### 5.3 索引管理器

```typescript
import { Index, Document } from 'flexsearch';
import RedisDB from 'flexsearch/db/redis';

export class IndexManager {
  private index: Index;
  private documentIndex: Document;
  private storage: RedisDB;
  private documentStore: Map<string, any>;

  constructor() {
    this.documentStore = new Map();
  }

  async initialize(): Promise<void> {
    this.index = new Index({
      tokenize: 'strict',
      resolution: 9,
      optimize: true,
      cache: true,
      async: true,
      worker: true
    });

    this.documentIndex = new Document({
      tokenize: 'strict',
      resolution: 9,
      optimize: true,
      cache: true
    });

    this.storage = new RedisDB('flexsearch', {
      url: process.env.REDIS_URL
    });

    await this.storage.open();
    await this.loadFromStorage();
  }

  async addDocument(doc: DocumentInput): Promise<void> {
    this.index.add(doc.id, doc.content);
    this.documentIndex.add(doc);

    if (doc.metadata) {
      this.documentStore.set(doc.id, {
        ...doc,
        metadata: doc.metadata
      });
    }

    await this.persistDocument(doc);
  }

  async updateDocument(doc: DocumentInput): Promise<void> {
    await this.removeDocument(doc.id);
    await this.addDocument(doc);
  }

  async removeDocument(id: string): Promise<void> {
    this.index.remove(id);
    this.documentIndex.remove(id);
    this.documentStore.delete(id);
    await this.storage.remove(id);
  }

  async search(query: string, limit: number = 10): Promise<string[]> {
    return this.index.search(query, limit);
  }

  async searchWithMetadata(
    query: string,
    limit: number = 10
  ): Promise<SearchResult[]> {
    const ids = await this.search(query, limit);
    return ids.map(id => ({
      id,
      doc: this.documentStore.get(id) || null,
      score: 1
    }));
  }

  async getStats(): Promise<IndexStats> {
    return {
      documentCount: this.documentStore.size,
      indexSize: JSON.stringify(this.index).length
    };
  }

  private async persistDocument(doc: DocumentInput): Promise<void> {
    await this.storage.set(doc.id, JSON.stringify(doc));
  }

  private async loadFromStorage(): Promise<void> {
    // 从存储加载文档
    // 这里可以实现增量加载或全量加载
  }
}
```

### 5.4 搜索管理器

```typescript
export class SearchManager {
  private indexManager: IndexManager;

  constructor(indexManager: IndexManager) {
    this.indexManager = indexManager;
  }

  async search(request: SearchRequest): Promise<SearchResultData> {
    const { query, limit, offset, options } = request;

    let results: SearchResult[];

    if (options?.resolution === 'advanced') {
      results = await this.indexManager.searchWithMetadata(query, limit);
    } else {
      const ids = await this.indexManager.search(query, limit);
      results = ids.map(id => ({ id, score: 1, doc: null }));
    }

    return {
      items: results.slice(offset, offset + limit),
      total: results.length
    };
  }

  highlight(results: SearchResult[], query: string): SearchResult[] {
    const terms = query.split(/\s+/).filter(t => t.length > 0);

    return results.map(result => {
      if (!result.doc) return result;

      const content = result.doc.content || '';
      const highlights: string[] = [];

      terms.forEach(term => {
        const regex = new RegExp(`(${term})`, 'gi');
        const matches = content.match(regex);
        if (matches) {
          highlights.push(...matches);
        }
      });

      return {
        ...result,
        highlights
      };
    });
  }

  async suggest(query: string, limit: number = 10): Promise<string[]> {
    const results = await this.indexManager.search(query, limit * 3);
    return results.slice(0, limit);
  }
}
```

### 5.5 缓存管理器

```typescript
import Redis from 'flexsearch/db/redis';

export class CacheManager {
  private redis: Redis;
  private ttl: number;

  constructor() {
    this.redis = new Redis('flexsearch-cache', {
      url: process.env.REDIS_URL
    });
    this.ttl = 300;  // 5 分钟
  }

  async initialize(): Promise<void> {
    await this.redis.open();
  }

  async get(key: string): Promise<any> {
    const value = await this.redis.get(key);
    if (value) {
      return JSON.parse(value);
    }
    return null;
  }

  async set(key: string, value: any, ttl?: number): Promise<void> {
    await this.redis.set(key, JSON.stringify(value));
    await this.redis.expire(key, ttl || this.ttl);
  }

  async delete(key: string): Promise<void> {
    await this.redis.del(key);
  }

  async clear(): Promise<void> {
    await this.redis.clear();
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
| - 实现服务器 | 1.5 天 | P0 | - |
| - 健康检查 | 1 天 | P0 | - |
| **索引管理器** | 3 天 | P0 | gRPC 服务 |
| - FlexSearch 集成 | 1 天 | P0 | - |
| - 文档管理 | 1 天 | P0 | - |
| - 持久化 | 1 天 | P0 | - |
| **搜索管理器** | 2 天 | P0 | 索引管理器 |
| - 搜索实现 | 1 天 | P0 | - |
| - 高亮显示 | 0.5 天 | P0 | - |
| - 建议功能 | 0.5 天 | P0 | - |
| **缓存管理器** | 2 天 | P1 | 搜索管理器 |
| - Redis 缓存 | 1 天 | P1 | - |
| - 缓存策略 | 1 天 | P1 | - |
| **批量操作** | 2 天 | P1 | 索引管理器 |
| - 批量添加 | 1 天 | P1 | - |
| - 批量删除 | 1 天 | P1 | - |
| **测试** | 3 天 | P1 | 所有功能 |
| - 单元测试 | 1.5 天 | P1 | - |
| - 集成测试 | 1 天 | P1 | - |
| - 压力测试 | 0.5 天 | P1 | - |
| **Docker 化** | 1 天 | P2 | 测试 |
| - Dockerfile | 0.5 天 | P2 | - |
| - docker-compose | 0.5 天 | P2 | - |
| **总计** | **20 天** | - | - |

### 6.2 里程碑

| 里程碑 | 交付物 | 时间 |
|--------|--------|------|
| **M1: gRPC 服务** | gRPC 服务器、协议定义、健康检查 | 第 4 天 |
| **M2: 索引管理器** | FlexSearch 集成、文档管理、持久化 | 第 7 天 |
| **M3: 搜索管理器** | 搜索实现、高亮显示、建议功能 | 第 9 天 |
| **M4: 缓存管理器** | Redis 缓存、缓存策略 | 第 11 天 |
| **M5: 批量操作** | 批量添加、批量删除 | 第 13 天 |
| **M6: 测试完成** | 单元测试、集成测试、压力测试 | 第 16 天 |
| **M7: 部署就绪** | Docker 镜像、文档 | 第 17 天 |

---

## 七、测试策略

### 7.1 单元测试

```typescript
import { FlexSearchService } from '../src/service';

describe('FlexSearchService', () => {
  let service: FlexSearchService;

  beforeEach(async () => {
    service = new FlexSearchService();
    await service.initialize();
  });

  afterEach(async () => {
    await service.close();
  });

  test('should add document', async () => {
    const request = {
      id: 'doc1',
      content: 'Hello world',
      metadata: { category: 'test' }
    };

    const response = await service.addDocument(request);
    expect(response.success).toBe(true);
  });

  test('should search documents', async () => {
    await service.addDocument({
      id: 'doc1',
      content: 'Hello world'
    });

    const response = await service.search({
      query: 'hello',
      limit: 10,
      offset: 0,
      options: {}
    });

    expect(response.results.length).toBeGreaterThan(0);
    expect(response.results[0].id).toBe('doc1');
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

EXPOSE 8083

CMD ["node", "dist/index.js"]
```

### 8.2 Docker Compose

```yaml
version: '3.8'

services:
  flexsearch:
    build: ./services/flexsearch
    ports:
      - "8083:8083"
    environment:
      - NODE_ENV=production
      - REDIS_URL=redis://redis:6379
      - CACHE_TTL=300
      - WORKER_COUNT=4
      - LOG_LEVEL=info
    depends_on:
      - redis
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "wget", "--quiet", "--tries=1", "--spider", "http://localhost:8083/health"]
      interval: 30s
      timeout: 10s
      retries: 3
```

---

## 九、监控和日志

### 9.1 监控指标

| 指标 | 类型 | 说明 |
|------|------|------|
| **document_count** | Gauge | 文档总数 |
| **search_total** | Counter | 搜索总数 |
| **search_latency** | Histogram | 搜索延迟 |
| **cache_hit** | Counter | 缓存命中数 |
| **cache_miss** | Counter | 缓存未命中数 |

### 9.2 日志格式

```json
{
  "level": "info",
  "time": "2024-01-01T00:00:00.000Z",
  "pid": 12345,
  "hostname": "flexsearch-1",
  "component": "flexsearch",
  "query": "hello",
  "results": 10,
  "latency": 5,
  "cache": "hit"
}
```

---

## 十、风险和缓解

| 风险 | 概率 | 影响 | 缓解措施 |
|------|------|------|---------|
| 内存占用高 | 中 | 高 | 优化索引、使用持久化 |
| 搜索延迟高 | 低 | 中 | 使用缓存、优化查询 |
| 持久化失败 | 低 | 高 | 使用可靠的存储后端 |

---

**文档版本**：1.0
**最后更新**：2026-02-04
**作者**：FlexSearch 技术分析团队
