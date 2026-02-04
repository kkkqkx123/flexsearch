# 向量搜索服务模块开发计划

## 一、模块概述

向量搜索服务提供基于向量嵌入的语义搜索功能，使用 Qdrant 作为向量数据库，支持 HNSW 索引和混合搜索。

---

## 二、功能需求

### 2.1 核心功能

| 功能 | 描述 | 优先级 |
|------|------|--------|
| **文档索引** | 添加、更新、删除文档及其向量 | P0 |
| **向量搜索** | 基于余弦相似度的向量搜索 | P0 |
| **嵌入生成** | 集成嵌入模型生成向量 | P0 |
| **混合搜索** | 关键词 + 向量的混合搜索 | P0 |
| **批量操作** | 支持批量添加和删除 | P1 |
| **向量量化** | 支持向量量化减少内存占用 | P1 |
| **多模型支持** | 支持多种嵌入模型 | P2 |
| **监控指标** | 收集性能指标 | P2 |

### 2.2 向量嵌入模型

| 模型 | 维度 | 速度 | 准确度 | 适用场景 |
|------|------|------|--------|---------|
| **all-MiniLM-L6-v2** | 384 | 快 | 高 | 通用场景 |
| **all-mpnet-base-v2** | 768 | 中 | 很高 | 高精度场景 |
| **paraphrase-multilingual-MiniLM** | 384 | 快 | 高 | 多语言 |
| **e5-large-v2** | 1024 | 慢 | 很高 | 高精度场景 |

---

## 三、技术选型

### 3.1 核心依赖

| 依赖 | 版本 | 用途 |
|------|------|------|
| **@qdrant/js-client-rest** | 最新 | Qdrant 客户端 |
| **@grpc/grpc-js** | 1.10.0 | gRPC 服务 |
| **transformers.js** | 2.x | 嵌入模型（浏览器） |
| **@xenova/transformers** | 2.x | 嵌入模型（Node.js） |
| **redis** | 4.7.0 | 缓存 |
| **pino** | 8.19.0 | 日志系统 |

### 3.2 依赖库

```json
{
  "dependencies": {
    "@qdrant/js-client-rest": "^1.7.0",
    "@grpc/grpc-js": "^1.10.0",
    "@grpc/proto-loader": "^0.7.10",
    "@xenova/transformers": "^2.16.0",
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
│         向量搜索服务         │
│  ┌──────────────┬──────────────┐       │
│  │ 索引管理器   │ 搜索管理器   │       │
│  └──────────────┴──────────────┘       │
│  ┌──────────────┬──────────────┐       │
│  │ 嵌入管理器   │ 缓存管理器   │       │
│  └──────────────┴──────────────┘       │
└──────────────┬──────────────────────┘
               │
        ┌──────┼──────┐
        │      │        │
        ▼      ▼        ▼
┌──────────┐ ┌──────────┐ ┌──────────┐
│  Qdrant  │ │  Redis   │ │ 嵌入模型  │
│  向量数据库│ │  缓存    │ │  (可选)  │
└──────────┘ └──────────┘ └──────────┘
```

### 4.2 目录结构

```
services/vector/
├── src/
│   ├── index.ts                 # 入口文件
│   ├── server.ts               # gRPC 服务器
│   ├── config/                 # 配置
│   │   ├── index.ts
│   │   └── qdrant.ts
│   ├── index/                  # 索引管理
│   │   ├── manager.ts
│   │   └── builder.ts
│   ├── search/                 # 搜索管理
│   │   ├── manager.ts
│   │   └── hybrid.ts
│   ├── embedding/              # 嵌入管理
│   │   ├── manager.ts
│   │   ├── models.ts
│   │   └── cache.ts
│   ├── cache/                  # 缓存管理
│   │   ├── index.ts
│   │   └── redis.ts
│   ├── types/                  # 类型定义
│   │   └── index.ts
│   └── utils/                  # 工具函数
│       ├── logger.ts
│       └── metrics.ts
├── proto/
│   └── vector.proto           # gRPC 协议定义
├── models/                    # 嵌入模型
│   └── README.md
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

package vector;

service VectorSearchService {
  rpc AddDocument(AddDocumentRequest) returns (AddDocumentResponse);
  rpc BatchAddDocuments(BatchAddDocumentsRequest) returns (BatchAddDocumentsResponse);
  rpc UpdateDocument(UpdateDocumentRequest) returns (UpdateDocumentResponse);
  rpc RemoveDocument(RemoveDocumentRequest) returns (RemoveDocumentResponse);
  rpc Search(SearchRequest) returns (SearchResponse);
  rpc HybridSearch(HybridSearchRequest) returns (SearchResponse);
  rpc GetStatus(StatusRequest) returns (StatusResponse);
}

message AddDocumentRequest {
  string id = 1;
  string content = 2;
  map<string, string> metadata = 3;
  repeated string keywords = 4;
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
  repeated string keywords = 4;
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
  string collection = 2;
  int32 limit = 3;
  int32 offset = 4;
  map<string, string> filters = 5;
  SearchOptions options = 6;
}

message SearchOptions {
  bool enrich = 1;
  string model = 2;
  float score_threshold = 3;
}

message HybridSearchRequest {
  string query = 1;
  repeated string keywords = 2;
  string collection = 3;
  int32 limit = 4;
  int32 offset = 5;
  map<string, string> filters = 6;
  HybridOptions options = 7;
}

message HybridOptions {
  float alpha = 1;  // 向量搜索权重
  float beta = 2;   // 关键词搜索权重
  bool enrich = 3;
  string model = 4;
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
  repeated string keywords = 4;
}

message Document {
  string id = 1;
  string content = 2;
  map<string, string> metadata = 3;
}

message StatusRequest {}

message StatusResponse {
  bool healthy = 1;
  int64 document_count = 2;
  int32 dimension = 3;
  string model = 4;
  map<string, string> metadata = 5;
}
```

### 5.2 向量搜索服务

```typescript
import { IndexManager } from './index/manager';
import { SearchManager } from './search/manager';
import { EmbeddingManager } from './embedding/manager';
import { CacheManager } from './cache';
import { Logger } from './utils/logger';
import { Metrics } from './utils/metrics';

export class VectorSearchService {
  private indexManager: IndexManager;
  private searchManager: SearchManager;
  private embeddingManager: EmbeddingManager;
  private cacheManager: CacheManager;
  private logger: Logger;
  private metrics: Metrics;

  constructor() {
    this.indexManager = new IndexManager();
    this.searchManager = new SearchManager(this.indexManager);
    this.embeddingManager = new EmbeddingManager();
    this.cacheManager = new CacheManager();
    this.logger = new Logger('vector');
    this.metrics = new Metrics();
  }

  async initialize(): Promise<void> {
    await this.indexManager.initialize();
    await this.embeddingManager.initialize();
    await this.cacheManager.initialize();
    this.logger.info('Vector search service initialized');
  }

  async addDocument(request: AddDocumentRequest): Promise<AddDocumentResponse> {
    const startTime = Date.now();

    try {
      const vector = await this.embeddingManager.generateEmbedding(
        request.content,
        request.options?.model
      );

      await this.indexManager.addDocument({
        id: request.id,
        content: request.content,
        vector,
        metadata: request.metadata,
        keywords: request.keywords
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
        await this.addDocument(doc);
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
      const vector = await this.embeddingManager.generateEmbedding(
        request.content,
        request.options?.model
      );

      await this.indexManager.removeDocument(request.id);
      await this.indexManager.addDocument({
        id: request.id,
        content: request.content,
        vector,
        metadata: request.metadata,
        keywords: request.keywords
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
      const cached = await this.cacheManager.get(cacheKey);
      if (cached) {
        this.logger.info('Cache hit', { query: request.query });
        this.metrics.increment('cache_hit');
        return cached;
      }

      const vector = await this.embeddingManager.generateEmbedding(
        request.query,
        request.options?.model
      );

      const results = await this.searchManager.search({
        vector,
        collection: request.collection || 'default',
        limit: request.limit,
        offset: request.offset,
        filters: request.filters,
        options: request.options
      });

      const response: SearchResponse = {
        results: results.items,
        total: results.total,
        latency: Date.now() - startTime,
        metadata: {
          engine: 'vector',
          model: request.options?.model || 'default'
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

  async hybridSearch(
    request: HybridSearchRequest
  ): Promise<SearchResponse> {
    const startTime = Date.now();
    const cacheKey = this.getHybridCacheKey(request);

    try {
      const cached = await this.cacheManager.get(cacheKey);
      if (cached) {
        this.logger.info('Cache hit', { query: request.query });
        this.metrics.increment('cache_hit');
        return cached;
      }

      const vector = await this.embeddingManager.generateEmbedding(
        request.query,
        request.options?.model
      );

      const results = await this.searchManager.hybridSearch({
        vector,
        keywords: request.keywords,
        collection: request.collection || 'default',
        limit: request.limit,
        offset: request.offset,
        filters: request.filters,
        options: {
          alpha: request.options?.alpha || 0.7,
          beta: request.options?.beta || 0.3,
          enrich: request.options?.enrich
        }
      });

      const response: SearchResponse = {
        results: results.items,
        total: results.total,
        latency: Date.now() - startTime,
        metadata: {
          engine: 'hybrid',
          model: request.options?.model || 'default'
        }
      };

      await this.cacheManager.set(cacheKey, response, 300);

      this.metrics.histogram('search_latency', response.latency);
      this.metrics.increment('search_total');

      return response;
    } catch (error) {
      this.logger.error('Hybrid search failed', { error, request });
      this.metrics.increment('search_error');
      throw error;
    }
  }

  async getStatus(): Promise<StatusResponse> {
    const stats = await this.indexManager.getStats();
    return {
      healthy: true,
      document_count: stats.documentCount,
      dimension: stats.dimension,
      model: this.embeddingManager.getCurrentModel(),
      metadata: {
        version: '1.0.0',
        uptime: process.uptime().toString()
      }
    };
  }

  private getCacheKey(request: SearchRequest): string {
    const hash = crypto
      .createHash('md5')
      .update(JSON.stringify(request))
      .digest('hex');
    return `vector:${hash}`;
  }

  private getHybridCacheKey(request: HybridSearchRequest): string {
    const hash = crypto
      .createHash('md5')
      .update(JSON.stringify(request))
      .digest('hex');
    return `hybrid:${hash}`;
  }
}
```

### 5.3 索引管理器

```typescript
import { QdrantClient } from '@qdrant/js-client-rest';

export class IndexManager {
  private client: QdrantClient;
  private collections: Map<string, CollectionInfo>;

  constructor() {
    this.client = new QdrantClient({
      url: process.env.QDRANT_URL || 'http://localhost:6333'
    });
    this.collections = new Map();
  }

  async initialize(): Promise<void> {
    await this.createDefaultCollection();
  }

  async createCollection(
    name: string,
    dimension: number = 384
  ): Promise<void> {
    await this.client.createCollection(name, {
      vectors: {
        size: dimension,
        distance: 'Cosine',
        hnsw_config: {
          m: 16,
          ef_construct: 200,
          full_scan_threshold: 10000
        }
      },
      optimizers_config: {
        indexing_threshold: 10000
      },
      replication_factor: 1
    });

    this.collections.set(name, {
      name,
      dimension,
      vectorCount: 0,
      status: 'green'
    });
  }

  async addDocument(doc: VectorDocument): Promise<void> {
    const collection = doc.collection || 'default';

    if (!this.collections.has(collection)) {
      await this.createCollection(collection, doc.vector.length);
    }

    await this.client.upsert(collection, {
      points: [
        {
          id: doc.id,
          vector: doc.vector,
          payload: {
            content: doc.content,
            metadata: doc.metadata || {},
            keywords: doc.keywords || []
          }
        }
      ]
    });

    const info = this.collections.get(collection)!;
    info.vectorCount++;
  }

  async removeDocument(id: string, collection: string = 'default'): Promise<void> {
    await this.client.delete(collection, {
      points: [id]
    });

    const info = this.collections.get(collection);
    if (info && info.vectorCount > 0) {
      info.vectorCount--;
    }
  }

  async getStats(collection: string = 'default'): Promise<IndexStats> {
    const info = await this.client.getCollection(collection);
    return {
      documentCount: info.result.points_count,
      dimension: info.result.config.params.vectors.size,
      vectorCount: info.result.points_count
    };
  }

  private async createDefaultCollection(): Promise<void> {
    try {
      await this.createCollection('default', 384);
    } catch (error) {
      if (!error.message.includes('already exists')) {
        throw error;
      }
    }
  }
}
```

### 5.4 嵌入管理器

```typescript
import { pipeline, env } from '@xenova/transformers';

export class EmbeddingManager {
  private models: Map<string, any>;
  private currentModel: string;
  private cache: Map<string, number[]>;

  constructor() {
    this.models = new Map();
    this.currentModel = 'Xenova/all-MiniLM-L6-v2';
    this.cache = new Map();
  }

  async initialize(): Promise<void> {
    await this.loadModel(this.currentModel);
  }

  async generateEmbedding(
    text: string,
    model?: string
  ): Promise<number[]> {
    const modelName = model || this.currentModel;
    const cacheKey = `${modelName}:${text}`;

    if (this.cache.has(cacheKey)) {
      return this.cache.get(cacheKey)!;
    }

    const embeddingModel = this.models.get(modelName);
    if (!embeddingModel) {
      await this.loadModel(modelName);
    }

    const output = await embeddingModel(text, {
      pooling: 'mean',
      normalize: true
    });

    const embedding = Array.from(output.data);
    this.cache.set(cacheKey, embedding);

    return embedding;
  }

  async loadModel(name: string): Promise<void> {
    if (this.models.has(name)) {
      return;
    }

    const model = await pipeline(
      'feature-extraction',
      name,
      {
        quantized: true,
        progress_callback: (progress) => {
          if (progress.status === 'progress') {
            console.log(`Loading model: ${progress.progress}%`);
          }
        }
      }
    );

    this.models.set(name, model);
    this.currentModel = name;
  }

  getCurrentModel(): string {
    return this.currentModel;
  }

  async switchModel(name: string): Promise<void> {
    await this.loadModel(name);
    this.currentModel = name;
  }

  clearCache(): void {
    this.cache.clear();
  }
}
```

### 5.5 搜索管理器

```typescript
import { QdrantClient } from '@qdrant/js-client-rest';

export class SearchManager {
  private client: QdrantClient;

  constructor(private indexManager: IndexManager) {
    this.client = new QdrantClient({
      url: process.env.QDRANT_URL || 'http://localhost:6333'
    });
  }

  async search(request: VectorSearchRequest): Promise<SearchResultData> {
    const { vector, collection, limit, offset, filters, options } = request;

    const queryFilter = this.buildFilter(filters);

    const results = await this.client.search(collection, {
      vector,
      limit: limit || 10,
      offset: offset || 0,
      with_payload: true,
      score_threshold: options?.score_threshold || 0,
      filter: queryFilter
    });

    return {
      items: results.map(r => ({
        id: r.id as string,
        score: r.score,
        doc: {
          id: r.id as string,
          content: r.payload?.content || '',
          metadata: r.payload?.metadata || {}
        },
        keywords: r.payload?.keywords || []
      })),
      total: results.length
    };
  }

  async hybridSearch(request: HybridSearchRequest): Promise<SearchResultData> {
    const { vector, keywords, collection, limit, offset, filters, options } = request;

    const alpha = options?.alpha || 0.7;
    const beta = options?.beta || 0.3;

    const queryFilter = this.buildFilter(filters);

    const vectorResults = await this.client.search(collection, {
      vector,
      limit: limit || 10,
      offset: offset || 0,
      with_payload: true,
      filter: queryFilter
    });

    const keywordResults = await this.client.scroll(collection, {
      limit: limit || 10,
      offset: offset || 0,
      with_payload: true,
      filter: {
        must: keywords.map(k => ({
          key: 'keywords',
          match: { value: k }
        }))
      }
    });

    const merged = this.mergeResults(
      vectorResults,
      keywordResults,
      alpha,
      beta
    );

    return {
      items: merged.slice(0, limit || 10),
      total: merged.length
    };
  }

  private mergeResults(
    vectorResults: any[],
    keywordResults: any[],
    alpha: number,
    beta: number
  ): SearchResult[] {
    const scores = new Map<string, number>();

    vectorResults.forEach(r => {
      const id = r.id as string;
      scores.set(id, r.score * alpha);
    });

    keywordResults.forEach(r => {
      const id = r.id as string;
      const score = scores.get(id) || 0;
      scores.set(id, score + beta);
    });

    return Array.from(scores.entries())
      .map(([id, score]) => ({
        id,
        score,
        doc: {
          id,
          content: '',
          metadata: {}
        },
        keywords: []
      }))
      .sort((a, b) => b.score - a.score);
  }

  private buildFilter(filters?: Map<string, string>): any {
    if (!filters || filters.size === 0) {
      return undefined;
    }

    const must: any[] = [];
    for (const [key, value] of filters.entries()) {
      must.push({
        key: `metadata.${key}`,
        match: { value }
      });
    }

    return { must };
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
| - Qdrant 集成 | 1 天 | P0 | - |
| - 集合管理 | 1 天 | P0 | - |
| - 文档管理 | 1 天 | P0 | - |
| **嵌入管理器** | 4 天 | P0 | 索引管理器 |
| - 模型加载 | 1.5 天 | P0 | - |
| - 嵌入生成 | 1.5 天 | P0 | - |
| - 模型缓存 | 1 天 | P0 | - |
| **搜索管理器** | 3 天 | P0 | 嵌入管理器 |
| - 向量搜索 | 1 天 | P0 | - |
| - 混合搜索 | 1.5 天 | P0 | - |
| - 结果融合 | 0.5 天 | P0 | - |
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
| **总计** | **25 天** | - | - |

### 6.2 里程碑

| 里程碑 | 交付物 | 时间 |
|--------|--------|------|
| **M1: gRPC 服务** | gRPC 服务器、协议定义、健康检查 | 第 4 天 |
| **M2: 索引管理器** | Qdrant 集成、集合管理、文档管理 | 第 7 天 |
| **M3: 嵌入管理器** | 模型加载、嵌入生成、模型缓存 | 第 11 天 |
| **M4: 搜索管理器** | 向量搜索、混合搜索、结果融合 | 第 14 天 |
| **M5: 缓存管理器** | Redis 缓存、缓存策略 | 第 16 天 |
| **M6: 批量操作** | 批量添加、批量删除 | 第 18 天 |
| **M7: 测试完成** | 单元测试、集成测试、压力测试 | 第 21 天 |
| **M8: 部署就绪** | Docker 镜像、文档 | 第 22 天 |

---

## 七、测试策略

### 7.1 单元测试

```typescript
import { VectorSearchService } from '../src/service';

describe('VectorSearchService', () => {
  let service: VectorSearchService;

  beforeEach(async () => {
    service = new VectorSearchService();
    await service.initialize();
  });

  afterEach(async () => {
    await service.close();
  });

  test('should add document', async () => {
    const request = {
      id: 'doc1',
      content: 'Hello world',
      metadata: { category: 'test' },
      keywords: ['hello', 'world']
    };

    const response = await service.addDocument(request);
    expect(response.success).toBe(true);
  });

  test('should search documents', async () => {
    await service.addDocument({
      id: 'doc1',
      content: 'Hello world',
      keywords: ['hello', 'world']
    });

    const response = await service.search({
      query: 'hello',
      collection: 'default',
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

EXPOSE 8085

CMD ["node", "dist/index.js"]
```

### 8.2 Docker Compose

```yaml
version: '3.8'

services:
  vector:
    build: ./services/vector
    ports:
      - "8085:8085"
    environment:
      - NODE_ENV=production
      - QDRANT_URL=http://qdrant:6333
      - REDIS_URL=redis://redis:6379
      - EMBEDDING_MODEL=Xenova/all-MiniLM-L6-v2
      - CACHE_TTL=300
      - LOG_LEVEL=info
    depends_on:
      - qdrant
      - redis
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "wget", "--quiet", "--tries=1", "--spider", "http://localhost:8085/health"]
      interval: 30s
      timeout: 10s
      retries: 3

  qdrant:
    image: qdrant/qdrant:latest
    ports:
      - "6333:6333"
      - "6334:6334"
    volumes:
      - ./qdrant_storage:/qdrant/storage
    restart: unless-stopped
```

---

## 九、监控和日志

### 9.1 监控指标

| 指标 | 类型 | 说明 |
|------|------|------|
| **document_count** | Gauge | 文档总数 |
| **search_total** | Counter | 搜索总数 |
| **search_latency** | Histogram | 搜索延迟 |
| **embedding_latency** | Histogram | 嵌入生成延迟 |
| **cache_hit** | Counter | 缓存命中数 |

### 9.2 日志格式

```json
{
  "level": "info",
  "time": "2024-01-01T00:00:00.000Z",
  "pid": 12345,
  "hostname": "vector-1",
  "component": "vector",
  "query": "hello",
  "model": "Xenova/all-MiniLM-L6-v2",
  "results": 10,
  "latency": 100,
  "cache": "hit"
}
```

---

## 十、风险和缓解

| 风险 | 概率 | 影响 | 缓解措施 |
|------|------|------|---------|
| 嵌入生成慢 | 中 | 高 | 使用缓存、优化模型 |
| Qdrant 故障 | 低 | 高 | 熔断机制、降级策略 |
| 内存占用高 | 中 | 中 | 向量量化、模型优化 |
| 缓存一致性 | 低 | 中 | 合理的缓存策略 |

---

**文档版本**：1.0
**最后更新**：2026-02-04
**作者**：FlexSearch 技术分析团队
