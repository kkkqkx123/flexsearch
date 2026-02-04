package handler

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

type HealthHandler struct{}

func NewHealthHandler() *HealthHandler {
	return &HealthHandler{}
}

func (h *HealthHandler) Check(c *gin.Context) {
	c.JSON(http.StatusOK, gin.H{
		"status": "healthy",
		"service": "api-gateway",
	})
}

func (h *HealthHandler) CheckServices(c *gin.Context) {
	c.JSON(http.StatusOK, gin.H{
		"status": "healthy",
		"services": gin.H{
			"coordinator": "not_implemented",
			"redis": "not_implemented",
		},
	})
}
