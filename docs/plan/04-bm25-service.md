# BM25 服务模块开发计划

## 一、模块概述

BM25 服务提供基于 BM25 算法的全文搜索功能，使用 PostgreSQL 作为存储后端，利用其原生全文搜索能力实现高性能的 BM25 搜索。

---

## 二、功能需求

### 2.1 核心功能

| 功能 | 描述 | 优先级 |
|------|------|--------|
| **文档索引** | 添加、更新、删除文档 | P0 |
| **BM25 搜索** | 基于 BM25 算法的全文搜索 | P0 |
| **词频统计** | 统计词频和文档频率 | P0 |
| **参数调优** | 支持 k1、b 参数调优 | P0 |
| **批量操作** | 支持批量添加和删除 | P1 |
| **高亮显示** | 搜索结果高亮 | P1 |
| **多语言支持** | 支持多种语言的分词 | P2 |
| **监控指标** | 收集性能指标 | P2 |

### 2.2 BM25 算法

BM25 是一种用于信息检索的概率排序函数，其公式为：

```
score(D, Q) = Σ IDF(qi) * (f(qi, D) * (k1 + 1)) / (f(qi, D) + k1 * (1 - b + b * |D| / avgdl))

其中：
- D: 文档
- Q: 查询
- qi: 查询中的第 i 个词
- f(qi, D): 词 qi 在文档 D 中的词频
- |D|: 文档 D 的长度
- avgdl: 平均文档长度
- k1: 词频饱和参数（通常为 1.2-2.0）
- b: 长度归一化参数（通常为 0.75）
- IDF(qi): 词 qi 的逆文档频率
```

---

## 三、技术选型

### 3.1 核心依赖

| 依赖 | 版本 | 用途 |
|------|------|------|
| **flexsearch/db/postgres** | 0.1.0 | PostgreSQL 客户端 |
| **@grpc/grpc-js** | 1.10.0 | gRPC 服务 |
| **redis** | 4.7.0 | 缓存 |
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
│         BM25 服务               │
│  ┌──────────────┬──────────────┐       │
│  │ 索引管理器   │ 搜索管理器   │       │
│  └──────────────┴──────────────┘       │
│  ┌──────────────┬──────────────┐       │
│  │ 统计管理器   │ 缓存管理器   │       │
│  └──────────────┴──────────────┘       │
└──────────────┬──────────────────────┘
               │
        ┌──────┼──────┐
        │      │        │
        ▼      ▼        ▼
┌──────────┐ ┌──────────┐ ┌──────────┐
│PostgreSQL│ │  Redis   │ │  文档存储 │
│ 全文搜索  │ │  缓存    │ │  (可选)  │
└──────────┘ └──────────┘ └──────────┘
```

### 4.2 目录结构

```
services/bm25/
├── src/
│   ├── index.ts                 # 入口文件
│   ├── server.ts               # gRPC 服务器
│   ├── config/                 # 配置
│   │   ├── index.ts
│   │   └── postgres.ts
│   ├── index/                  # 索引管理
│   │   ├── manager.ts
│   │   └── builder.ts
│   ├── search/                 # 搜索管理
│   │   ├── manager.ts
│   │   └── scorer.ts
│   ├── stats/                  # 统计管理
│   │   ├── manager.ts
│   │   └── calculator.ts
│   ├── cache/                  # 缓存管理
│   │   ├── index.ts
│   │   └── redis.ts
│   ├── types/                  # 类型定义
│   │   └── index.ts
│   └── utils/                  # 工具函数
│       ├── logger.ts
│       └── metrics.ts
├── proto/
│   └── bm25.proto             # gRPC 协议定义
├── sql/
│   ├── 001_init.sql            # 初始化脚本
│   └── 002_indexes.sql        # 索引脚本
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

package bm25;

service BM25Service {
  rpc AddDocument(AddDocumentRequest) returns (AddDocumentResponse);
  rpc BatchAddDocuments(BatchAddDocumentsRequest) returns (BatchAddDocumentsResponse);
  rpc UpdateDocument(UpdateDocumentRequest) returns (UpdateDocumentResponse);
  rpc RemoveDocument(RemoveDocumentRequest) returns (RemoveDocumentResponse);
  rpc Search(SearchRequest) returns (SearchResponse);
  rpc GetStats(StatsRequest) returns (StatsResponse);
  rpc UpdateParameters(ParametersRequest) returns (ParametersResponse);
}

message AddDocumentRequest {
  string id = 1;
  string title = 2;
  string content = 3;
  map<string, string> metadata = 4;
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
  string title = 2;
  string content = 3;
  map<string, string> metadata = 4;
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
  string language = 3;
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
  string title = 2;
  string content = 3;
  map<string, string> metadata = 4;
}

message StatsRequest {}

message StatsResponse {
  int64 document_count = 1;
  int64 total_terms = 2;
  float avg_doc_length = 3;
  BM25Parameters parameters = 4;
}

message BM25Parameters {
  float k1 = 1;
  float b = 2;
}

message ParametersRequest {
  float k1 = 1;
  float b = 2;
}

message ParametersResponse {
  bool success = 1;
  BM25Parameters parameters = 2;
}
```

### 5.2 BM25 服务

```typescript
import { IndexManager } from './index/manager';
import { SearchManager } from './search/manager';
import { StatsManager } from './stats/manager';
import { CacheManager } from './cache';
import { Logger } from './utils/logger';
import { Metrics } from './utils/metrics';

export class BM25Service {
  private indexManager: IndexManager;
  private searchManager: SearchManager;
  private statsManager: StatsManager;
  private cacheManager: CacheManager;
  private logger: Logger;
  private metrics: Metrics;

  constructor() {
    this.indexManager = new IndexManager();
    this.searchManager = new SearchManager(this.indexManager);
    this.statsManager = new StatsManager();
    this.cacheManager = new CacheManager();
    this.logger = new Logger('bm25');
    this.metrics = new Metrics();
  }

  async initialize(): Promise<void> {
    await this.indexManager.initialize();
    await this.statsManager.initialize();
    await this.cacheManager.initialize();
    this.logger.info('BM25 service initialized');
  }

  async addDocument(request: AddDocumentRequest): Promise<AddDocumentResponse> {
    const startTime = Date.now();

    try {
      await this.indexManager.addDocument({
        id: request.id,
        title: request.title,
        content: request.content,
        metadata: request.metadata
      });

      await this.statsManager.updateDocumentStats(request.id, {
        title: request.title,
        content: request.content
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
        await this.statsManager.updateDocumentStats(doc.id, {
          title: doc.title,
          content: doc.content
        });
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
      await this.indexManager.removeDocument(request.id);
      await this.indexManager.addDocument({
        id: request.id,
        title: request.title,
        content: request.content,
        metadata: request.metadata
      });

      await this.statsManager.updateDocumentStats(request.id, {
        title: request.title,
        content: request.content
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
      await this.statsManager.removeDocumentStats(request.id);

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
      const cached = await this.cacheManager.get(cacheKey);
      if (cached) {
        this.logger.info('Cache hit', { query: request.query });
        this.metrics.increment('cache_hit');
        return cached;
      }

      const results = await this.searchManager.search(request);

      if (request.options?.highlight) {
        results.results = this.searchManager.highlight(results.results, request.query);
      }

      const response: SearchResponse = {
        results: results.items,
        total: results.total,
        latency: Date.now() - startTime,
        metadata: {
          engine: 'bm25',
          cached: false
        }
      };

      await this.cacheManager.set(cacheKey, response, 300);

      this.metrics.histogram('search_latency', response.latency);
      this.metrics.increment('search_total');

      return response;
    } catch (error) {
      this.logger.error('Search failed', { error, request });
      this.metrics.increment('search_error');
      throw error;
    }
  }

  async getStats(): Promise<StatsResponse> {
    const stats = await this.statsManager.getStats();
    return {
      document_count: stats.documentCount,
      total_terms: stats.totalTerms,
      avg_doc_length: stats.avgDocLength,
      parameters: {
        k1: this.searchManager.getK1(),
        b: this.searchManager.getB()
      }
    };
  }

  async updateParameters(request: ParametersRequest): Promise<ParametersResponse> {
    this.searchManager.setK1(request.k1 || 1.2);
    this.searchManager.setB(request.b || 0.75);

    return {
      success: true,
      parameters: {
        k1: this.searchManager.getK1(),
        b: this.searchManager.getB()
      }
    };
  }

  private getCacheKey(request: SearchRequest): string {
    const hash = crypto
      .createHash('md5')
      .update(JSON.stringify(request))
      .digest('hex');
    return `bm25:${hash}`;
  }
}
```

### 5.3 索引管理器

```typescript
import PostgresDB from 'flexsearch/db/postgres';

export class IndexManager {
  private db: PostgresDB;

  constructor() {
    this.db = new PostgresDB('bm25', {
      host: process.env.POSTGRES_HOST,
      port: process.env.POSTGRES_PORT,
      user: process.env.POSTGRES_USER,
      pass: process.env.POSTGRES_PASS,
      name: process.env.POSTGRES_DB
    });
  }

  async initialize(): Promise<void> {
    await this.db.open();
    await this.createTables();
    await this.createIndexes();
  }

  async addDocument(doc: DocumentInput): Promise<void> {
    await this.db.query(`
      INSERT INTO documents (id, title, content, metadata)
      VALUES ($1, $2, $3, $4)
      ON CONFLICT (id) DO UPDATE SET
        title = EXCLUDED.title,
        content = EXCLUDED.content,
        metadata = EXCLUDED.metadata
    `, [doc.id, doc.title, doc.content, JSON.stringify(doc.metadata || {})]);
  }

  async removeDocument(id: string): Promise<void> {
    await this.db.query('DELETE FROM documents WHERE id = $1', [id]);
  }

  async getDocument(id: string): Promise<Document | null> {
    const result = await this.db.oneOrNone(`
      SELECT id, title, content, metadata
      FROM documents
      WHERE id = $1
    `, [id]);

    if (!result) return null;

    return {
      id: result.id,
      title: result.title,
      content: result.content,
      metadata: JSON.parse(result.metadata || '{}')
    };
  }

  private async createTables(): Promise<void> {
    await this.db.query(`
      CREATE TABLE IF NOT EXISTS documents (
        id VARCHAR(255) PRIMARY KEY,
        title TEXT,
        content TEXT,
        metadata JSONB,
        title_vector tsvector,
        content_vector tsvector,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
      );
    `);
  }

  private async createIndexes(): Promise<void> {
    await this.db.query(`
      CREATE INDEX IF NOT EXISTS idx_documents_title_vector
      ON documents USING GIN (title_vector);
    `);

    await this.db.query(`
      CREATE INDEX IF NOT EXISTS idx_documents_content_vector
      ON documents USING GIN (content_vector);
    `);

    await this.db.query(`
      CREATE OR REPLACE FUNCTION documents_search_update()
      RETURNS TRIGGER AS $$
      BEGIN
        NEW.title_vector := to_tsvector('english', COALESCE(NEW.title, ''));
        NEW.content_vector := to_tsvector('english', COALESCE(NEW.content, ''));
        RETURN NEW;
      END;
      $$ LANGUAGE plpgsql;
    `);

    await this.db.query(`
      DROP TRIGGER IF EXISTS documents_search_trigger ON documents;
    `);

    await this.db.query(`
      CREATE TRIGGER documents_search_trigger
        BEFORE INSERT OR UPDATE ON documents
        FOR EACH ROW EXECUTE FUNCTION documents_search_update();
    `);
  }
}
```

### 5.4 搜索管理器

```typescript
export class SearchManager {
  private indexManager: IndexManager;
  private k1: number = 1.2;
  private b: number = 0.75;

  constructor(indexManager: IndexManager) {
    this.indexManager = indexManager;
  }

  async search(request: SearchRequest): Promise<SearchResultData> {
    const { query, limit, offset, options } = request;
    const language = options?.language || 'english';

    const queryVector = await this.indexManager.db.one(`
      SELECT to_tsquery('${language}', $1) as query
    `, [query]);

    const results = await this.indexManager.db.manyOrNone(`
      SELECT
        id,
        title,
        content,
        metadata,
        ts_rank_cd(title_vector, $1, 1) * 0.7 +
        ts_rank_cd(content_vector, $1, 1) * 0.3 as score
      FROM documents
      WHERE title_vector @@ $1 OR content_vector @@ $1
      ORDER BY score DESC
      LIMIT $2 OFFSET $3
    `, [queryVector.query, limit, offset]);

    const total = await this.indexManager.db.one(`
      SELECT COUNT(*) as count
      FROM documents
      WHERE title_vector @@ $1 OR content_vector @@ $1
    `, [queryVector.query]);

    return {
      items: results.map(r => ({
        id: r.id,
        score: r.score,
        doc: {
          id: r.id,
          title: r.title,
          content: r.content,
          metadata: JSON.parse(r.metadata || '{}')
        },
        highlights: []
      })),
      total: parseInt(total.count)
    };
  }

  highlight(results: SearchResult[], query: string): SearchResult[] {
    const terms = query.split(/\s+/).filter(t => t.length > 0);

    return results.map(result => {
      if (!result.doc) return result;

      const title = result.doc.title || '';
      const content = result.doc.content || '';
      const highlights: string[] = [];

      terms.forEach(term => {
        const regex = new RegExp(`(${term})`, 'gi');
        const titleMatches = title.match(regex);
        const contentMatches = content.match(regex);

        if (titleMatches) {
          highlights.push(...titleMatches);
        }
        if (contentMatches) {
          highlights.push(...contentMatches);
        }
      });

      return {
        ...result,
        highlights
      };
    });
  }

  setK1(k1: number): void {
    this.k1 = k1;
  }

  getK1(): number {
    return this.k1;
  }

  setB(b: number): void {
    this.b = b;
  }

  getB(): number {
    return this.b;
  }
}
```

### 5.5 统计管理器

```typescript
import PostgresDB from 'flexsearch/db/postgres';

export class StatsManager {
  private db: PostgresDB;

  constructor() {
    this.db = new PostgresDB('bm25', {
      host: process.env.POSTGRES_HOST,
      port: process.env.POSTGRES_PORT,
      user: process.env.POSTGRES_USER,
      pass: process.env.POSTGRES_PASS,
      name: process.env.POSTGRES_DB
    });
  }

  async initialize(): Promise<void> {
    await this.createStatsTables();
  }

  async updateDocumentStats(id: string, doc: { title: string; content: string }): Promise<void> {
    const terms = this.extractTerms(`${doc.title} ${doc.content}`);
    const docLength = terms.length;

    await this.db.query(`
      INSERT INTO document_stats (id, length, terms)
      VALUES ($1, $2, $3)
      ON CONFLICT (id) DO UPDATE SET
        length = EXCLUDED.length,
        terms = EXCLUDED.terms
    `, [id, docLength, JSON.stringify(terms)]);

    for (const term of terms) {
      await this.db.query(`
        INSERT INTO term_stats (term, doc_freq, total_freq)
        VALUES ($1, 1, 1)
        ON CONFLICT (term) DO UPDATE SET
          doc_freq = term_stats.doc_freq + 1,
          total_freq = term_stats.total_freq + 1
      `, [term]);
    }
  }

  async removeDocumentStats(id: string): Promise<void> {
    const stats = await this.db.oneOrNone(`
      SELECT terms FROM document_stats WHERE id = $1
    `, [id]);

    if (stats) {
      const terms = JSON.parse(stats.terms);
      for (const term of terms) {
        await this.db.query(`
          UPDATE term_stats
          SET doc_freq = doc_freq - 1,
               total_freq = total_freq - 1
          WHERE term = $1
        `, [term]);
      }
    }

    await this.db.query('DELETE FROM document_stats WHERE id = $1', [id]);
  }

  async getStats(): Promise<BM25Stats> {
    const documentCount = await this.db.one(`
      SELECT COUNT(*) as count FROM documents
    `);

    const totalTerms = await this.db.one(`
      SELECT SUM(total_freq) as sum FROM term_stats
    `);

    const avgDocLength = await this.db.one(`
      SELECT AVG(length) as avg FROM document_stats
    `);

    return {
      documentCount: parseInt(documentCount.count),
      totalTerms: parseInt(totalTerms.sum || 0),
      avgDocLength: parseFloat(avgDocLength.avg || 0)
    };
  }

  private extractTerms(text: string): string[] {
    return text
      .toLowerCase()
      .split(/\s+/)
      .filter(t => t.length > 2);
  }

  private async createStatsTables(): Promise<void> {
    await this.db.query(`
      CREATE TABLE IF NOT EXISTS document_stats (
        id VARCHAR(255) PRIMARY KEY,
        length INTEGER,
        terms JSONB
      );
    `);

    await this.db.query(`
      CREATE TABLE IF NOT EXISTS term_stats (
        term VARCHAR(255) PRIMARY KEY,
        doc_freq INTEGER,
        total_freq INTEGER
      );
    `);
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
| - PostgreSQL 集成 | 1 天 | P0 | - |
| - 文档管理 | 1 天 | P0 | - |
| - 全文搜索配置 | 1 天 | P0 | - |
| **搜索管理器** | 3 天 | P0 | 索引管理器 |
| - BM25 搜索 | 1.5 天 | P0 | - |
| - 高亮显示 | 0.5 天 | P0 | - |
| - 参数调优 | 1 天 | P0 | - |
| **统计管理器** | 3 天 | P0 | 搜索管理器 |
| - 词频统计 | 1.5 天 | P0 | - |
| - 文档统计 | 1.5 天 | P0 | - |
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
| **总计** | **24 天** | - | - |

### 6.2 里程碑

| 里程碑 | 交付物 | 时间 |
|--------|--------|------|
| **M1: gRPC 服务** | gRPC 服务器、协议定义、健康检查 | 第 4 天 |
| **M2: 索引管理器** | PostgreSQL 集成、文档管理、全文搜索 | 第 7 天 |
| **M3: 搜索管理器** | BM25 搜索、高亮显示、参数调优 | 第 10 天 |
| **M4: 统计管理器** | 词频统计、文档统计 | 第 13 天 |
| **M5: 缓存管理器** | Redis 缓存、缓存策略 | 第 15 天 |
| **M6: 批量操作** | 批量添加、批量删除 | 第 17 天 |
| **M7: 测试完成** | 单元测试、集成测试、压力测试 | 第 20 天 |
| **M8: 部署就绪** | Docker 镜像、文档 | 第 21 天 |

---

## 七、测试策略

### 7.1 单元测试

```typescript
import { BM25Service } from '../src/service';

describe('BM25Service', () => {
  let service: BM25Service;

  beforeEach(async () => {
    service = new BM25Service();
    await service.initialize();
  });

  afterEach(async () => {
    await service.close();
  });

  test('should add document', async () => {
    const request = {
      id: 'doc1',
      title: 'Test Document',
      content: 'This is a test document',
      metadata: { category: 'test' }
    };

    const response = await service.addDocument(request);
    expect(response.success).toBe(true);
  });

  test('should search documents', async () => {
    await service.addDocument({
      id: 'doc1',
      title: 'Test Document',
      content: 'This is a test document'
    });

    const response = await service.search({
      query: 'test',
      limit: 10,
      offset: 0,
      options: {}
    });

    expect(response.results.length).toBeGreaterThan(0);
    expect(response.results[0].score).toBeGreaterThan(0);
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

EXPOSE 8084

CMD ["node", "dist/index.js"]
```

### 8.2 Docker Compose

```yaml
version: '3.8'

services:
  bm25:
    build: ./services/bm25
    ports:
      - "8084:8084"
    environment:
      - NODE_ENV=production
      - POSTGRES_HOST=postgres
      - POSTGRES_PORT=5432
      - POSTGRES_USER=postgres
      - POSTGRES_PASS=postgres
      - POSTGRES_DB=search
      - REDIS_URL=redis://redis:6379
      - K1=1.2
      - B=0.75
      - LOG_LEVEL=info
    depends_on:
      - postgres
      - redis
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "wget", "--quiet", "--tries=1", "--spider", "http://localhost:8084/health"]
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
  "hostname": "bm25-1",
  "component": "bm25",
  "query": "test",
  "results": 10,
  "latency": 50,
  "k1": 1.2,
  "b": 0.75
}
```

---

## 十、风险和缓解

| 风险 | 概率 | 影响 | 缓解措施 |
|------|------|------|---------|
| PostgreSQL 性能瓶颈 | 中 | 高 | 优化查询、使用索引 |
| 统计计算慢 | 中 | 中 | 增量更新统计 |
| 缓存一致性 | 低 | 中 | 合理的缓存策略 |

---

**文档版本**：1.0
**最后更新**：2026-02-04
**作者**：FlexSearch 技术分析团队
