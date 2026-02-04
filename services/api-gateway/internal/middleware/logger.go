package middleware

import (
	"fmt"
	"time"

	"github.com/gin-gonic/gin"
	"github.com/google/uuid"
)

func Logger() gin.HandlerFunc {
	return func(c *gin.Context) {
		start := time.Now()
		path := c.Request.URL.Path
		query := c.Request.URL.RawQuery

		requestID := c.GetHeader("X-Request-ID")
		if requestID == "" {
			requestID = uuid.New().String()
		}
		c.Set("RequestID", requestID)
		c.Header("X-Request-ID", requestID)

		c.Next()

		latency := time.Since(start)
		status := c.Writer.Status()
		method := c.Request.Method
		clientIP := c.ClientIP()

		if query != "" {
			path = path + "?" + query
		}

		gin.DefaultWriter.Write([]byte(fmt.Sprintf(
			"[%s] %s %s %s %d %v %s\n",
			time.Now().Format("2006-01-02 15:04:05"),
			requestID,
			method,
			path,
			status,
			latency,
			clientIP,
		)))
	}
}
