package handler

import (
	"context"
	"net/http"
	"strconv"
	"time"

	"github.com/flexsearch/api-gateway/internal/client"
	"github.com/flexsearch/api-gateway/internal/config"
	"github.com/flexsearch/api-gateway/internal/util"
	pb "github.com/flexsearch/api-gateway/proto"
	"github.com/gin-gonic/gin"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/trace"
	"go.uber.org/zap"
)

type HealthHandler struct {
	client *client.CoordinatorClient
	config *config.Config
	logger *zap.Logger
	tracer trace.Tracer
}

func NewHealthHandler(client *client.CoordinatorClient, cfg *config.Config, logger *zap.Logger) *HealthHandler {
	return &HealthHandler{
		client: client,
		config: cfg,
		logger: logger,
		tracer: otel.Tracer("health-handler"),
	}
}

func (h *HealthHandler) Check(c *gin.Context) {
	c.JSON(http.StatusOK, gin.H{
		"status":    "healthy",
		"service":   "api-gateway",
		"timestamp": time.Now().UTC().Format(time.RFC3339),
		"version":   "1.0.0",
	})
}

func (h *HealthHandler) CheckServices(c *gin.Context) {
	ctx := c.Request.Context()
	ctx, cancel := context.WithTimeout(ctx, 10*time.Second)
	defer cancel()

	services := make(map[string]interface{})
	overallStatus := "healthy"

	coordinatorStatus := h.checkCoordinator(ctx)
	services["coordinator"] = coordinatorStatus
	if coordinatorStatus["status"] != "healthy" {
		overallStatus = "unhealthy"
	}

	c.JSON(http.StatusOK, gin.H{
		"status":   overallStatus,
		"services": services,
		"timestamp": time.Now().UTC().Format(time.RFC3339),
	})
}

func (h *HealthHandler) checkCoordinator(ctx context.Context) map[string]interface{} {
	start := time.Now()

	req := &pb.HealthCheckRequest{
		Service: "coordinator",
	}

	resp, err := h.client.HealthCheck(ctx, req)
	latency := time.Since(start)

	if err != nil {
		h.logger.Error("Coordinator health check failed", zap.Error(err))
		return map[string]interface{}{
			"status":     "unhealthy",
			"latency_ms": latency.Milliseconds(),
			"address":    h.config.Coordinator.Address,
			"error":      err.Error(),
		}
	}

	return map[string]interface{}{
		"status":         resp.Status,
		"version":        resp.Version,
		"uptime_seconds": resp.UptimeSeconds,
		"latency_ms":     latency.Milliseconds(),
		"address":        h.config.Coordinator.Address,
		"details":        resp.Details,
	}
}
