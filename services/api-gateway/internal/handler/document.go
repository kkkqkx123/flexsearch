package handler

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

func (h *DocumentHandler) Create(c *gin.Context) {
	c.JSON(http.StatusOK, gin.H{
		"message": "Create document endpoint - not implemented yet",
	})
}

func (h *DocumentHandler) Get(c *gin.Context) {
	id := c.Param("id")
	c.JSON(http.StatusOK, gin.H{
		"message": "Get document endpoint - not implemented yet",
		"id":      id,
	})
}

func (h *DocumentHandler) Update(c *gin.Context) {
	id := c.Param("id")
	c.JSON(http.StatusOK, gin.H{
		"message": "Update document endpoint - not implemented yet",
		"id":      id,
	})
}

func (h *DocumentHandler) Delete(c *gin.Context) {
	id := c.Param("id")
	c.JSON(http.StatusOK, gin.H{
		"message": "Delete document endpoint - not implemented yet",
		"id":      id,
	})
}

func (h *DocumentHandler) Batch(c *gin.Context) {
	c.JSON(http.StatusOK, gin.H{
		"message": "Batch documents endpoint - not implemented yet",
	})
}
