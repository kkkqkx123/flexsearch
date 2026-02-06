module github.com/flexsearch/coordinator

go 1.21

require (
	github.com/flexsearch/shared v0.1.0
	github.com/prometheus/client_golang v1.19.1
	github.com/redis/go-redis/v9 v9.17.3
	github.com/spf13/viper v1.21.0
	go.opentelemetry.io/otel v1.24.0
	go.opentelemetry.io/otel/exporters/stdout/stdouttrace v1.24.0
	go.opentelemetry.io/otel/sdk v1.24.0
	go.opentelemetry.io/otel/trace v1.24.0
	go.uber.org/zap v1.27.0
	google.golang.org/grpc v1.62.1
	google.golang.org/protobuf v1.33.0
)
