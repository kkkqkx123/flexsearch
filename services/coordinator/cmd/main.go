package main

import (
	"context"
	"fmt"
	"net"
	"net/http"
	"os"
	"os/signal"
	"syscall"
	"time"

	"github.com/flexsearch/coordinator/internal/config"
	coordinatorServer "github.com/flexsearch/coordinator/internal/server"
	"github.com/flexsearch/coordinator/internal/util"
	"github.com/prometheus/client_golang/prometheus/promhttp"
	"google.golang.org/grpc"
	"google.golang.org/grpc/health"
	"google.golang.org/grpc/health/grpc_health_v1"
	"google.golang.org/grpc/reflection"
)

const (
	serviceName = "coordinator"
)

func main() {
	cfg, err := config.Load("configs/config.yaml")
	if err != nil {
		fmt.Fprintf(os.Stderr, "Failed to load config: %v\n", err)
		os.Exit(1)
	}

	logger, err := util.NewLogger(cfg.Logging.Level, cfg.Logging.Format, cfg.Logging.Output)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Failed to initialize logger: %v\n", err)
		os.Exit(1)
	}
	defer logger.Sync()

	metrics := util.NewMetrics(serviceName)

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	grpcServer := setupGRPCServer(cfg, logger)
	metricsServer := setupMetricsServer(cfg, metrics)

	if cfg.Metrics.Enabled {
		go func() {
			addr := cfg.GetMetricsAddress()
			logger.Infof("Starting metrics server on %s", addr)
			if err := metricsServer.ListenAndServe(); err != nil && err != http.ErrServerClosed {
				logger.Errorf("Metrics server error: %v", err)
			}
		}()
	}

	go func() {
		addr := cfg.GetGRPCAddress()
		logger.Infof("Starting gRPC server on %s", addr)
		listener, err := net.Listen("tcp", addr)
		if err != nil {
			logger.Fatalf("Failed to listen: %v", err)
		}
		if err := grpcServer.Serve(listener); err != nil {
			logger.Fatalf("Failed to serve: %v", err)
		}
	}()

	logger.Infof("%s service started successfully", serviceName)

	waitForShutdown(ctx, cancel, cfg, grpcServer, metricsServer, logger)
}

func setupGRPCServer(cfg *config.Config, logger *util.Logger) *grpc.Server {
	opts := []grpc.ServerOption{
		grpc.MaxRecvMsgSize(cfg.GRPC.MaxRecvMsgSize),
		grpc.MaxSendMsgSize(cfg.GRPC.MaxSendMsgSize),
	}

	server := grpc.NewServer(opts...)

	coordinator := coordinatorServer.NewCoordinatorServer(logger)

	healthServer := health.NewServer()
	healthServer.SetServingStatus("", grpc_health_v1.HealthCheckResponse_SERVING)
	healthServer.SetServingStatus(serviceName, grpc_health_v1.HealthCheckResponse_SERVING)
	grpc_health_v1.RegisterHealthCheckServer(server, healthServer)

	reflection.Register(server)

	logger.Info("gRPC server configured")
	return server
}

func setupMetricsServer(cfg *config.Config, metrics *util.Metrics) *http.Server {
	mux := http.NewServeMux()
	mux.Handle(cfg.Metrics.Path, promhttp.Handler())

	return &http.Server{
		Addr:    cfg.GetMetricsAddress(),
		Handler: mux,
	}
}

func waitForShutdown(ctx context.Context, cancel context.CancelFunc, cfg *config.Config, grpcServer *grpc.Server, metricsServer *http.Server, logger *util.Logger) {
	sigChan := make(chan os.Signal, 1)
	signal.Notify(sigChan, syscall.SIGINT, syscall.SIGTERM)

	sig := <-sigChan
	logger.Infof("Received signal: %v", sig)

	cancel()

	shutdownCtx, shutdownCancel := context.WithTimeout(context.Background(), cfg.Server.ShutdownTimeout)
	defer shutdownCancel()

	if metricsServer != nil && cfg.Metrics.Enabled {
		logger.Info("Shutting down metrics server...")
		if err := metricsServer.Shutdown(shutdownCtx); err != nil {
			logger.Errorf("Metrics server shutdown error: %v", err)
		}
	}

	logger.Info("Shutting down gRPC server...")
	stopped := make(chan struct{})
	go func() {
		grpcServer.GracefulStop()
		close(stopped)
	}()

	select {
	case <-stopped:
		logger.Info("gRPC server stopped gracefully")
	case <-shutdownCtx.Done():
		logger.Warn("gRPC server shutdown timeout, forcing stop...")
		grpcServer.Stop()
	}

	logger.Infof("%s service stopped", serviceName)
}
