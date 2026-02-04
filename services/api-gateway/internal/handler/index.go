package handler

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

func (h *IndexHandler) Create(c *gin.Context) {
	c.JSON(http.StatusOK, gin.H{
		"message": "Create index endpoint - not implemented yet",
	})
}

func (h *IndexHandler) List(c *gin.Context) {
	c.JSON(http.StatusOK, gin.H{
		"message": "List indexes endpoint - not implemented yet",
	})
}

func (h *IndexHandler) Get(c *gin.Context) {
	id := c.Param("id")
	c.JSON(http.StatusOK, gin.H{
		"message": "Get index endpoint - not implemented yet",
		"id":      id,
	})
}

func (h *IndexHandler) Delete(c *gin.Context) {
	id := c.Param("id")
	c.JSON(http.StatusOK, gin.H{
		"message": "Delete index endpoint - not implemented yet",
		"id":      id,
	})
}

func (h *IndexHandler) Rebuild(c *gin.Context) {
	id := c.Param("id")
	c.JSON(http.StatusOK, gin.H{
		"message": "Rebuild index endpoint - not implemented yet",
		"id":      id,
	})
}
