package middleware

import (
    "fmt"
    "net/http"
    "time"

    "github.com/gin-gonic/gin"
    "github.com/flexsearch/api-gateway/internal/util"
)

type RateLimitConfig struct {
    Limit  int
    Window time.Duration
    ByUser bool
    ByIP   bool
}

func RateLimitMiddleware(limiter *util.RateLimiter, config RateLimitConfig) gin.HandlerFunc {
    return func(c *gin.Context) {
        var key string
        if config.ByUser {
            userID := c.GetString("user_id")
            if userID != "" {
                key = fmt.Sprintf("ratelimit:user:%s", userID)
            } else {
                key = fmt.Sprintf("ratelimit:ip:%s", c.ClientIP())
            }
        } else if config.ByIP {
            key = fmt.Sprintf("ratelimit:ip:%s", c.ClientIP())
        } else {
            key = fmt.Sprintf("ratelimit:global")
        }

        allowed, err := limiter.Allow(c.Request.Context(), key, config.Limit, config.Window)
        if err != nil {
            c.JSON(http.StatusInternalServerError, gin.H{"error": "Rate limit error", "details": err.Error()})
            c.Abort()
            return
        }

        if !allowed {
            c.JSON(http.StatusTooManyRequests, gin.H{
                "error": "Rate limit exceeded",
                "limit": config.Limit,
                "window": config.Window.String(),
            })
            c.Abort()
            return
        }

        c.Next()
    }
}
