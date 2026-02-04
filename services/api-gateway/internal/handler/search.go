package handler

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

type SearchHandler struct{}

func NewSearchHandler() *SearchHandler {
	return &SearchHandler{}
}

func (h *SearchHandler) Search(c *gin.Context) {
	c.JSON(http.StatusOK, gin.H{
		"message": "Search endpoint - not implemented yet",
		"query":   c.Query("query"),
	})
}
