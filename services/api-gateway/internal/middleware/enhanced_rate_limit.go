package middleware

import (
	"context"
	"fmt"
	"net/http"
	"strings"
	"time"

	"github.com/flexsearch/api-gateway/internal/util"
	"github.com/gin-gonic/gin"
)

// EnhancedRateLimitConfig holds the configuration for enhanced rate limiting
type EnhancedRateLimitConfig struct {
	Enabled       bool
	DefaultLimit  int
	DefaultBurst  int
	DefaultWindow string
	ByUser        bool
	ByIP          bool
	HeaderBased   bool // Rate limit based on custom header
	HeaderName    string
	TierHeader    string // Header to determine user tier
}

// EnhancedRateLimitMiddleware creates a new enhanced rate limit middleware
func EnhancedRateLimitMiddleware(limiter *util.EnhancedRateLimiter, config EnhancedRateLimitConfig) gin.HandlerFunc {
	return func(c *gin.Context) {
		if !config.Enabled {
			c.Next()
			return
		}

		// Determine the rate limiting key
		key := determineRateLimitKey(c, config)

		// Determine user tier
		tier := determineUserTier(c, config)

		// Check rate limit
		allowed, err := limiter.Allow(c.Request.Context(), key, tier)
		if err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{
				"error":   "Rate limit error",
				"details": err.Error(),
			})
			c.Abort()
			return
		}

		if !allowed {
			tierConfig := util.TierConfig{
				Limit:  limiter.GetConfig().DefaultLimit,
				Burst:  limiter.GetConfig().DefaultBurst,
				Window: limiter.GetConfig().DefaultWindow,
			}

			c.JSON(http.StatusTooManyRequests, gin.H{
				"error":       "Rate limit exceeded",
				"limit":       tierConfig.Limit,
				"burst":       tierConfig.Burst,
				"window":      tierConfig.Window.String(),
				"tier":        string(tier),
				"retry_after": tierConfig.Window.Seconds(),
			})
			c.Abort()
			return
		}

		tierConfig := util.TierConfig{
			Limit:  limiter.GetConfig().DefaultLimit,
			Burst:  limiter.GetConfig().DefaultBurst,
			Window: limiter.GetConfig().DefaultWindow,
		}

		c.Header("X-RateLimit-Limit", fmt.Sprintf("%d", tierConfig.Limit))
		c.Header("X-RateLimit-Remaining", fmt.Sprintf("%d", getRemainingTokens(c.Request.Context(), limiter, key, tier)))
		c.Header("X-RateLimit-Reset", getResetTime(tierConfig.Window))
		c.Header("X-RateLimit-Tier", string(tier))

		c.Next()
	}
}

// determineRateLimitKey determines the rate limiting key based on configuration
func determineRateLimitKey(c *gin.Context, config EnhancedRateLimitConfig) string {
	if config.HeaderBased && config.HeaderName != "" {
		if headerValue := c.GetHeader(config.HeaderName); headerValue != "" {
			return fmt.Sprintf("header:%s:%s", config.HeaderName, headerValue)
		}
	}

	if config.ByUser {
		if userID := c.GetString("user_id"); userID != "" {
			return fmt.Sprintf("user:%s", userID)
		}
	}

	if config.ByIP {
		return fmt.Sprintf("ip:%s", c.ClientIP())
	}

	return "global"
}

// determineUserTier determines the user's rate limiting tier
func determineUserTier(c *gin.Context, config EnhancedRateLimitConfig) util.RateLimitTier {
	// Check if tier is specified in header
	if config.TierHeader != "" {
		if tierStr := c.GetHeader(config.TierHeader); tierStr != "" {
			tier := util.RateLimitTier(strings.ToLower(tierStr))
			if isValidTier(tier) {
				return tier
			}
		}
	}

	// Check if tier is specified in user context (from JWT claims)
	if userTier := c.GetString("rate_limit_tier"); userTier != "" {
		tier := util.RateLimitTier(strings.ToLower(userTier))
		if isValidTier(tier) {
			return tier
		}
	}

	// Check if user has premium role
	if roles := c.GetStringSlice("user_roles"); len(roles) > 0 {
		for _, role := range roles {
			if strings.Contains(strings.ToLower(role), "enterprise") {
				return util.TierEnterprise
			} else if strings.Contains(strings.ToLower(role), "premium") {
				return util.TierPremium
			} else if strings.Contains(strings.ToLower(role), "basic") {
				return util.TierBasic
			}
		}
	}

	// Default to free tier
	return util.TierFree
}

// isValidTier checks if the tier is valid
func isValidTier(tier util.RateLimitTier) bool {
	switch tier {
	case util.TierFree, util.TierBasic, util.TierPremium, util.TierEnterprise:
		return true
	default:
		return false
	}
}

// getRemainingTokens calculates remaining tokens (simplified)
func getRemainingTokens(ctx context.Context, limiter *util.EnhancedRateLimiter, key string, tier util.RateLimitTier) int {
	// This is a simplified implementation
	// In a real implementation, you would get the actual remaining tokens from the rate limiter
	tierConfig, exists := limiter.GetConfig().Tiers[tier]
	if !exists {
		tierConfig = util.TierConfig{
			Limit: limiter.GetConfig().DefaultLimit,
			Burst: limiter.GetConfig().DefaultBurst,
		}
	}

	// Return a reasonable estimate (in real implementation, get from Redis)
	return tierConfig.Burst / 2 // Placeholder
}

// getResetTime calculates the reset time for rate limit window
func getResetTime(window time.Duration) string {
	resetTime := time.Now().Add(window).Unix()
	return fmt.Sprintf("%d", resetTime)
}
