# API 网关模块开发计划

## 一、模块概述

API 网关是整个系统的入口，负责接收外部请求、认证授权、限流熔断、路由分发等功能。

---

## 二、功能需求

### 2.1 核心功能

| 功能 | 描述 | 优先级 |
|------|------|--------|
| **请求路由** | 根据请求路径和类型路由到不同的服务 | P0 |
| **认证授权** | JWT Token 验证、API Key 认证 | P0 |
| **限流熔断** | 基于用户/IP 的限流、服务熔断 | P0 |
| **日志记录** | 请求日志、响应日志、错误日志 | P0 |
| **健康检查** | 网关自身和后端服务的健康检查 | P1 |
| **请求/响应转换** | 格式转换、数据过滤 | P1 |
| **缓存** | 热门查询结果缓存 | P2 |
| **监控指标** | 请求量、延迟、错误率等指标 | P2 |

### 2.2 API 接口

#### 搜索接口
```
POST /api/v1/search
GET  /api/v1/search

请求体：
{
  "query": "搜索关键词",
  "limit": 10,
  "offset": 0,
  "filters": {
    "category": "tech",
    "date": "2024-01-01"
  },
  "options": {
    "engine": "auto",  // auto, flexsearch, bm25, vector, hybrid
    "enrich": true,
    "highlight": true
  }
}

响应：
{
  "results": [
    {
      "id": "doc1",
      "score": 0.95,
      "doc": {...},
      "highlights": [...]
    }
  ],
  "total": 100,
  "latency": 50,
  "engine": "flexsearch"
}
```

#### 文档管理接口
```
POST   /api/v1/documents
GET    /api/v1/documents/:id
PUT    /api/v1/documents/:id
DELETE /api/v1/documents/:id
POST   /api/v1/documents/batch
```

#### 索引管理接口
```
POST   /api/v1/indexes
GET    /api/v1/indexes
GET    /api/v1/indexes/:id
DELETE /api/v1/indexes/:id
POST   /api/v1/indexes/:id/rebuild
```

#### 健康检查接口
```
GET /health
GET /health/services
```

---

## 三、技术选型

### 3.1 框架选择

| 框架 | 优势 | 劣势 | 推荐度 |
|------|------|------|--------|
| **Express** | 生态丰富、学习曲线低 | 性能一般 | ⭐⭐⭐⭐ |
| **Fastify** | 高性能、类型安全 | 生态相对较小 | ⭐⭐⭐⭐⭐ |
| **Koa** | 轻量级、中间件机制 | 性能一般 | ⭐⭐⭐ |
| **NestJS** | 企业级、TypeScript 原生 | 学习曲线陡峭 | ⭐⭐⭐⭐ |

**推荐选择：Fastify**

**理由**：
- ✅ 高性能（比 Express 快 2 倍）
- ✅ 原生 TypeScript 支持
- ✅ 内置 JSON Schema 验证
- ✅ 丰富的插件生态
- ✅ 低内存占用

### 3.2 依赖库

```json
{
  "dependencies": {
    "fastify": "^4.25.0",
    "@fastify/cors": "^8.5.0",
    "@fastify/helmet": "^11.1.1",
    "@fastify/rate-limit": "^9.1.0",
    "@fastify/jwt": "^7.2.4",
    "@fastify/swagger": "^8.14.0",
    "@fastify/swagger-ui": "^2.1.0",
    "axios": "^1.6.5",
    "pino": "^8.19.0",
    "pino-pretty": "^10.3.1",
    "dotenv": "^16.3.1",
    "zod": "^3.22.4"
  },
  "devDependencies": {
    "@types/node": "^20.11.0",
    "typescript": "^5.3.3",
    "ts-node": "^10.9.2",
    "nodemon": "^3.0.3",
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
│         客户端                  │
└──────────────┬──────────────────────┘
               │ HTTP/HTTPS
               ▼
┌─────────────────────────────────────────┐
│         API 网关               │
│  ┌──────────────┬──────────────┐       │
│  │ 中间件层     │ 路由层       │       │
│  └──────────────┴──────────────┘       │
│  ┌──────────────┬──────────────┐       │
│  │ 认证授权     │ 限流熔断     │       │
│  └──────────────┴──────────────┘       │
└──────────────┬──────────────────────┘
               │ gRPC/HTTP
        ┌──────┼──────┬──────────┐
        │      │        │          │
        ▼      ▼        ▼          ▼
┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐
│查询协调器│ │配置服务  │ │认证服务  │ │监控服务  │
└──────────┘ └──────────┘ └──────────┘ └──────────┘
```

### 4.2 目录结构

```
services/api-gateway/
├── src/
│   ├── index.ts                 # 入口文件
│   ├── app.ts                  # Fastify 应用配置
│   ├── config/                 # 配置
│   │   ├── index.ts
│   │   ├── routes.ts
│   │   └── services.ts
│   ├── routes/                 # 路由
│   │   ├── search.ts
│   │   ├── documents.ts
│   │   ├── indexes.ts
│   │   └── health.ts
│   ├── middleware/             # 中间件
│   │   ├── auth.ts
│   │   ├── rate-limit.ts
│   │   ├── logger.ts
│   │   ├── error-handler.ts
│   │   └── validation.ts
│   ├── services/               # 服务
│   │   ├── coordinator-client.ts
│   │   ├── auth-service.ts
│   │   └── cache-service.ts
│   ├── schemas/                # Schema 验证
│   │   ├── search.ts
│   │   ├── document.ts
│   │   └── index.ts
│   ├── types/                  # 类型定义
│   │   └── index.ts
│   └── utils/                  # 工具函数
│       ├── logger.ts
│       └── error.ts
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

### 5.1 应用入口

```typescript
import Fastify from 'fastify';
import { config } from './config';
import { routes } from './config/routes';
import { authMiddleware } from './middleware/auth';
import { rateLimitMiddleware } from './middleware/rate-limit';
import { loggerMiddleware } from './middleware/logger';
import { errorHandler } from './middleware/error-handler';

export async function buildApp() {
  const app = Fastify({
    logger: {
      level: config.logLevel,
      transport: {
        target: 'pino-pretty',
        options: {
          colorize: true,
          translateTime: 'HH:MM:ss Z',
          ignore: 'pid,hostname'
        }
      }
    }
  });

  // 注册插件
  await app.register(import('@fastify/cors'), {
    origin: config.corsOrigin
  });

  await app.register(import('@fastify/helmet'));

  await app.register(import('@fastify/rate-limit'), {
    max: config.rateLimit.max,
    timeWindow: config.rateLimit.timeWindow
  });

  await app.register(import('@fastify/jwt'), {
    secret: config.jwtSecret
  });

  await app.register(import('@fastify/swagger'), {
    openapi: {
      info: {
        title: 'Search API',
        version: '1.0.0'
      }
    }
  });

  await app.register(import('@fastify/swagger-ui'), {
    routePrefix: '/docs'
  });

  // 注册中间件
  app.addHook('onRequest', loggerMiddleware);
  app.addHook('onRequest', authMiddleware);
  app.addHook('onRequest', rateLimitMiddleware);
  app.setErrorHandler(errorHandler);

  // 注册路由
  routes.forEach(route => {
    app.route(route);
  });

  // 健康检查
  app.get('/health', async (request, reply) => {
    return { status: 'ok', timestamp: new Date().toISOString() };
  });

  return app;
}
```

### 5.2 认证中间件

```typescript
import { FastifyRequest, FastifyReply } from 'fastify';

export async function authMiddleware(
  request: FastifyRequest,
  reply: FastifyReply
) {
  // 跳过健康检查和公开接口
  const publicPaths = ['/health', '/docs'];
  if (publicPaths.some(path => request.url.startsWith(path))) {
    return;
  }

  try {
    await request.jwtVerify();
  } catch (err) {
    reply.code(401).send({
      error: 'Unauthorized',
      message: 'Invalid or missing token'
    });
  }
}
```

### 5.3 限流中间件

```typescript
import { FastifyRequest, FastifyReply } from 'fastify';
import Redis from 'flexsearch/db/redis';

const redis = new Redis('rate-limit', {
  url: process.env.REDIS_URL
});

export async function rateLimitMiddleware(
  request: FastifyRequest,
  reply: FastifyReply
) {
  const userId = request.user?.id || request.ip;
  const key = `rate-limit:${userId}`;
  const limit = 100;  // 每分钟 100 次请求
  const window = 60;   // 60 秒

  const current = await redis.get(key);
  const count = current ? parseInt(current) : 0;

  if (count >= limit) {
    reply.code(429).send({
      error: 'Too Many Requests',
      message: 'Rate limit exceeded'
    });
    return;
  }

  await redis.set(key, count + 1, 'EX', window);
}
```

### 5.4 搜索路由

```typescript
import { FastifyInstance } from 'fastify';
import { searchSchema } from '../schemas/search';
import { CoordinatorClient } from '../services/coordinator-client';

export async function searchRoutes(fastify: FastifyInstance) {
  const coordinator = new CoordinatorClient();

  fastify.post('/api/v1/search', {
    schema: searchSchema
  }, async (request, reply) => {
    const { query, limit, offset, filters, options } = request.body as any;

    const startTime = Date.now();
    const results = await coordinator.search({
      query,
      limit,
      offset,
      filters,
      options
    });
    const latency = Date.now() - startTime;

    return {
      results: results.items,
      total: results.total,
      latency,
      engine: results.engine,
      metadata: results.metadata
    };
  });
}
```

### 5.5 Schema 验证

```typescript
import { Type, Static } from '@sinclair/typebox';

export const searchSchema = {
  body: Type.Object({
    query: Type.String({ minLength: 1 }),
    limit: Type.Optional(Type.Number({ minimum: 1, maximum: 100, default: 10 })),
    offset: Type.Optional(Type.Number({ minimum: 0, default: 0 })),
    filters: Type.Optional(Type.Object({
      category: Type.Optional(Type.String()),
      date: Type.Optional(Type.String())
    })),
    options: Type.Optional(Type.Object({
      engine: Type.Optional(Type.Union([
        Type.Literal('auto'),
        Type.Literal('flexsearch'),
        Type.Literal('bm25'),
        Type.Literal('vector'),
        Type.Literal('hybrid')
      ])),
      enrich: Type.Optional(Type.Boolean({ default: true })),
      highlight: Type.Optional(Type.Boolean({ default: true }))
    }))
  })
};
```

---

## 六、开发计划

### 6.1 任务分解

| 任务 | 预估时间 | 优先级 | 依赖 |
|------|---------|--------|------|
| **项目初始化** | 1 天 | P0 | - |
| - 创建项目结构 | 0.5 天 | P0 | - |
| - 配置 TypeScript | 0.5 天 | P0 | - |
| **基础框架搭建** | 2 天 | P0 | 项目初始化 |
| - Fastify 应用配置 | 0.5 天 | P0 | - |
| - 日志系统 | 0.5 天 | P0 | - |
| - 错误处理 | 0.5 天 | P0 | - |
| - Swagger 文档 | 0.5 天 | P0 | - |
| **中间件开发** | 3 天 | P0 | 基础框架 |
| - 认证中间件 | 1 天 | P0 | - |
| - 限流中间件 | 1 天 | P0 | - |
| - 日志中间件 | 0.5 天 | P0 | - |
| - 验证中间件 | 0.5 天 | P0 | - |
| **路由开发** | 3 天 | P0 | 中间件 |
| - 搜索路由 | 1 天 | P0 | - |
| - 文档路由 | 1 天 | P0 | - |
| - 索引路由 | 0.5 天 | P0 | - |
| - 健康检查路由 | 0.5 天 | P0 | - |
| **服务客户端** | 2 天 | P1 | 路由 |
| - 查询协调器客户端 | 1 天 | P1 | - |
| - 认证服务客户端 | 0.5 天 | P1 | - |
| - 缓存服务客户端 | 0.5 天 | P1 | - |
| **测试** | 3 天 | P1 | 所有功能 |
| - 单元测试 | 1 天 | P1 | - |
| - 集成测试 | 1 天 | P1 | - |
| - 压力测试 | 1 天 | P1 | - |
| **Docker 化** | 1 天 | P2 | 测试 |
| - Dockerfile | 0.5 天 | P2 | - |
| - docker-compose | 0.5 天 | P2 | - |
| **总计** | **15 天** | - | - |

### 6.2 里程碑

| 里程碑 | 交付物 | 时间 |
|--------|--------|------|
| **M1: 基础框架** | Fastify 应用、日志系统、错误处理 | 第 3 天 |
| **M2: 中间件完成** | 认证、限流、日志、验证中间件 | 第 6 天 |
| **M3: 路由完成** | 搜索、文档、索引、健康检查路由 | 第 9 天 |
| **M4: 测试完成** | 单元测试、集成测试、压力测试 | 第 12 天 |
| **M5: 部署就绪** | Docker 镜像、文档 | 第 15 天 |

---

## 七、测试策略

### 7.1 单元测试

```typescript
import { buildApp } from '../src/app';

describe('API Gateway', () => {
  let app;

  beforeAll(async () => {
    app = await buildApp();
  });

  afterAll(async () => {
    await app.close();
  });

  test('health check', async () => {
    const response = await app.inject({
      method: 'GET',
      url: '/health'
    });

    expect(response.statusCode).toBe(200);
    expect(response.json()).toMatchObject({
      status: 'ok'
    });
  });

  test('search endpoint', async () => {
    const response = await app.inject({
      method: 'POST',
      url: '/api/v1/search',
      headers: {
        authorization: 'Bearer valid-token'
      },
      payload: {
        query: 'test',
        limit: 10
      }
    });

    expect(response.statusCode).toBe(200);
    expect(response.json()).toHaveProperty('results');
  });
});
```

### 7.2 集成测试

```typescript
describe('Integration Tests', () => {
  test('end-to-end search', async () => {
    const response = await app.inject({
      method: 'POST',
      url: '/api/v1/search',
      payload: {
        query: 'flexsearch',
        limit: 10
      }
    });

    expect(response.statusCode).toBe(200);
    const data = response.json();
    expect(data.results).toBeInstanceOf(Array);
    expect(data.latency).toBeGreaterThan(0);
  });
});
```

### 7.3 压力测试

使用 Artillery 或 k6 进行压力测试：

```yaml
# load-test.yml
config:
  target: 'http://localhost:8080'
  phases:
    - duration: 60
      arrivalRate: 10
    - duration: 120
      arrivalRate: 50
    - duration: 60
      arrivalRate: 100

scenarios:
  - name: 'Search Load Test'
    flow:
      - post:
          url: '/api/v1/search'
          json:
            query: 'test'
            limit: 10
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

EXPOSE 8080

CMD ["node", "dist/index.js"]
```

### 8.2 Docker Compose

```yaml
version: '3.8'

services:
  api-gateway:
    build: ./services/api-gateway
    ports:
      - "8080:8080"
    environment:
      - NODE_ENV=production
      - COORDINATOR_URL=http://coordinator:8081
      - AUTH_URL=http://auth:8082
      - REDIS_URL=redis://redis:6379
      - JWT_SECRET=${JWT_SECRET}
      - LOG_LEVEL=info
    depends_on:
      - coordinator
      - auth
      - redis
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "wget", "--quiet", "--tries=1", "--spider", "http://localhost:8080/health"]
      interval: 30s
      timeout: 10s
      retries: 3
```

---

## 九、监控和日志

### 9.1 日志格式

```json
{
  "level": "info",
  "time": "2024-01-01T00:00:00.000Z",
  "pid": 12345,
  "hostname": "api-gateway-1",
  "reqId": "abc123",
  "method": "POST",
  "url": "/api/v1/search",
  "statusCode": 200,
  "responseTime": 50,
  "userId": "user123"
}
```

### 9.2 监控指标

| 指标 | 类型 | 说明 |
|------|------|------|
| **request_total** | Counter | 请求总数 |
| **request_duration** | Histogram | 请求延迟 |
| **request_errors** | Counter | 错误数 |
| **active_connections** | Gauge | 活跃连接数 |
| **rate_limit_hits** | Counter | 限流命中数 |

---

## 十、风险和缓解

| 风险 | 概率 | 影响 | 缓解措施 |
|------|------|------|---------|
| 性能瓶颈 | 中 | 高 | 压力测试、优化关键路径 |
| 认证服务故障 | 低 | 高 | 熔断机制、降级策略 |
| 限流误判 | 中 | 中 | 动态调整限流阈值 |
| 日志量过大 | 中 | 低 | 日志采样、异步写入 |

---

**文档版本**：1.0
**最后更新**：2026-02-04
**作者**：FlexSearch 技术分析团队
