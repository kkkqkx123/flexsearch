package util

import (
	"time"

	"github.com/flexsearch/metrics"
	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/promauto"
	"sync"
)

type Metrics struct {
	grpcRequestsTotal    *prometheus.CounterVec
	grpcRequestsDuration *prometheus.HistogramVec
	grpcRequestsInFlight prometheus.Gauge
	queryLatency         *prometheus.HistogramVec
	engineLatency        *prometheus.HistogramVec
	mergerLatency        *prometheus.HistogramVec
	startTime            time.Time
	mu                   sync.RWMutex
	redisMetrics         *metrics.RedisMetrics
	searchMetrics        *metrics.SearchMetrics
}

func NewMetrics(namespace string) *Metrics {
	m := &Metrics{
		grpcRequestsTotal: promauto.NewCounterVec(
			prometheus.CounterOpts{
				Namespace: namespace,
				Name:      "grpc_requests_total",
				Help:      "Total number of gRPC requests",
			},
			[]string{"method", "status"},
		),
		grpcRequestsDuration: promauto.NewHistogramVec(
			prometheus.HistogramOpts{
				Namespace: namespace,
				Name:      "grpc_request_duration_seconds",
				Help:      "gRPC request duration in seconds",
				Buckets:   prometheus.DefBuckets,
			},
			[]string{"method"},
		),
		grpcRequestsInFlight: promauto.NewGauge(
			prometheus.GaugeOpts{
				Namespace: namespace,
				Name:      "grpc_requests_in_flight",
				Help:      "Number of gRPC requests currently being processed",
			},
		),
		queryLatency: promauto.NewHistogramVec(
			prometheus.HistogramOpts{
				Namespace: namespace,
				Name:      "query_latency_seconds",
				Help:      "Query operation latency in seconds",
				Buckets:   prometheus.DefBuckets,
			},
			[]string{"query_type"},
		),
		engineLatency: promauto.NewHistogramVec(
			prometheus.HistogramOpts{
				Namespace: namespace,
				Name:      "engine_latency_seconds",
				Help:      "Search engine latency in seconds",
				Buckets:   prometheus.DefBuckets,
			},
			[]string{"engine", "operation"},
		),
		mergerLatency: promauto.NewHistogramVec(
			prometheus.HistogramOpts{
				Namespace: namespace,
				Name:      "merger_latency_seconds",
				Help:      "Result merger latency in seconds",
				Buckets:   prometheus.DefBuckets,
			},
			[]string{"strategy"},
		),
		startTime:    time.Now(),
		redisMetrics: metrics.NewRedisMetrics("coordinator", "default"),
		searchMetrics: metrics.NewSearchMetrics("coordinator"),
	}

	return m
}

func (m *Metrics) IncrementGRPCRequest(method, status string) {
	m.grpcRequestsTotal.WithLabelValues(method, status).Inc()
}

func (m *Metrics) RecordGRPCDuration(method string, duration time.Duration) {
	m.grpcRequestsDuration.WithLabelValues(method).Observe(duration.Seconds())
}

func (m *Metrics) IncrementInFlight() {
	m.grpcRequestsInFlight.Inc()
}

func (m *Metrics) DecrementInFlight() {
	m.grpcRequestsInFlight.Dec()
}

func (m *Metrics) RecordQueryLatency(queryType string, duration time.Duration) {
	m.queryLatency.WithLabelValues(queryType).Observe(duration.Seconds())
}

func (m *Metrics) RecordEngineLatency(engine, operation string, duration time.Duration) {
	m.engineLatency.WithLabelValues(engine, operation).Observe(duration.Seconds())
	m.searchMetrics.RecordDuration(engine, duration.Seconds())
}

func (m *Metrics) IncrementCacheHit(cacheType string) {
	m.redisMetrics.RecordCacheHit(cacheType)
}

func (m *Metrics) IncrementCacheMiss(cacheType string) {
	m.redisMetrics.RecordCacheMiss(cacheType)
}

func (m *Metrics) RecordMergerLatency(strategy string, duration time.Duration) {
	m.mergerLatency.WithLabelValues(strategy).Observe(duration.Seconds())
}

func (m *Metrics) GetUptime() time.Duration {
	return time.Since(m.startTime)
}

func (m *Metrics) RecordRedisOperation(operation, status string, duration float64) {
	m.redisMetrics.RecordOperation(operation, status, duration)
}

func (m *Metrics) RecordRedisConnection(active, idle int) {
	m.redisMetrics.RecordConnection(active, idle)
}

func (m *Metrics) RecordSearchRequest(engine string) {
	m.searchMetrics.RecordRequest(engine)
}

func (m *Metrics) RecordSearchResults(engine string, count float64) {
	m.searchMetrics.RecordResults(engine, count)
}

func (m *Metrics) RecordSearchError(engine, errorType string) {
	m.searchMetrics.RecordError(engine, errorType)
}

func (m *Metrics) RedisMetrics() *metrics.RedisMetrics {
	return m.redisMetrics
}

func (m *Metrics) SearchMetrics() *metrics.SearchMetrics {
	return m.searchMetrics
}
