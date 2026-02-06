package util

import (
	"context"
	"fmt"
	"sync"
	"time"

	"github.com/redis/go-redis/v9"
)

// RateLimitTier represents different rate limit tiers
type RateLimitTier string

const (
	TierFree       RateLimitTier = "free"
	TierBasic      RateLimitTier = "basic"
	TierPremium    RateLimitTier = "premium"
	TierEnterprise RateLimitTier = "enterprise"
)

// RateLimitConfig holds the configuration for rate limiting
type EnhancedRateLimitConfig struct {
	Enabled       bool
	DefaultLimit  int
	DefaultBurst  int
	DefaultWindow time.Duration
	ByUser        bool
	ByIP          bool
	Tiers         map[RateLimitTier]TierConfig
	RedisPrefix   string
}

// TierConfig holds configuration for a specific tier
type TierConfig struct {
	Limit  int
	Burst  int
	Window time.Duration
}

// DefaultEnhancedRateLimitConfig returns a default enhanced configuration
func DefaultEnhancedRateLimitConfig() EnhancedRateLimitConfig {
	return EnhancedRateLimitConfig{
		Enabled:       true,
		DefaultLimit:  100,
		DefaultBurst:  20,
		DefaultWindow: time.Minute,
		ByUser:        true,
		ByIP:          true,
		RedisPrefix:   "ratelimit",
		Tiers: map[RateLimitTier]TierConfig{
			TierFree: {
				Limit:  60,
				Burst:  10,
				Window: time.Minute,
			},
			TierBasic: {
				Limit:  300,
				Burst:  50,
				Window: time.Minute,
			},
			TierPremium: {
				Limit:  1000,
				Burst:  200,
				Window: time.Minute,
			},
			TierEnterprise: {
				Limit:  5000,
				Burst:  1000,
				Window: time.Minute,
			},
		},
	}
}

// EnhancedRateLimiter provides advanced rate limiting with burst and tiers
type EnhancedRateLimiter struct {
	redis  *redis.Client
	config EnhancedRateLimitConfig
	mu     sync.RWMutex
}

// NewEnhancedRateLimiter creates a new enhanced rate limiter
func NewEnhancedRateLimiter(redisClient *redis.Client, config EnhancedRateLimitConfig) *EnhancedRateLimiter {
	return &EnhancedRateLimiter{
		redis:  redisClient,
		config: config,
	}
}

// Allow checks if a request should be allowed based on rate limiting rules
func (rl *EnhancedRateLimiter) Allow(ctx context.Context, key string, tier RateLimitTier) (bool, error) {
	if !rl.config.Enabled {
		return true, nil
	}

	tierConfig, exists := rl.config.Tiers[tier]
	if !exists {
		tierConfig = TierConfig{
			Limit:  rl.config.DefaultLimit,
			Burst:  rl.config.DefaultBurst,
			Window: rl.config.DefaultWindow,
		}
	}

	return rl.allowRequest(ctx, key, tierConfig)
}

// allowRequest implements the token bucket algorithm
func (rl *EnhancedRateLimiter) allowRequest(ctx context.Context, key string, config TierConfig) (bool, error) {
	bucketKey := fmt.Sprintf("%s:bucket:%s", rl.config.RedisPrefix, key)

	// Get current bucket state
	pipe := rl.redis.Pipeline()
	getCmd := pipe.Get(ctx, bucketKey)

	_, err := pipe.Exec(ctx)
	if err != nil && err != redis.Nil {
		return false, err
	}

	var tokens int
	var lastRefill time.Time

	if err == redis.Nil {
		// Initialize bucket
		tokens = config.Burst
		lastRefill = time.Now()
	} else {
		// Parse existing bucket state
		value, _ := getCmd.Result()
		if value != "" {
			// Parse tokens and last refill time from stored value
			parts := []byte(value)
			if len(parts) >= 16 {
				tokens = int(int64(parts[0]) | int64(parts[1])<<8 | int64(parts[2])<<16 | int64(parts[3])<<24)
				lastRefill = time.Unix(int64(parts[4])|int64(parts[5])<<8|int64(parts[6])<<16|int64(parts[7])<<24,
					int64(parts[8])|int64(parts[9])<<8|int64(parts[10])<<16|int64(parts[11])<<24)
			}
		}
	}

	// Calculate tokens to add based on time elapsed
	now := time.Now()
	elapsed := now.Sub(lastRefill)
	tokensToAdd := int(elapsed.Seconds() * float64(config.Limit) / config.Window.Seconds())

	if tokensToAdd > 0 {
		tokens = min(tokens+tokensToAdd, config.Burst)
		lastRefill = now
	}

	// Check if we can consume a token
	if tokens > 0 {
		tokens--

		// Store updated bucket state
		value := fmt.Sprintf("%d:%d", tokens, lastRefill.Unix())
		err := rl.redis.Set(ctx, bucketKey, value, config.Window).Err()
		if err != nil {
			return false, err
		}

		return true, nil
	}

	// No tokens available, but still update the bucket state
	value := fmt.Sprintf("%d:%d", tokens, lastRefill.Unix())
	err = rl.redis.Set(ctx, bucketKey, value, config.Window).Err()
	if err != nil {
		return false, err
	}

	return false, nil
}

// GetStats returns rate limiting statistics for a key
func (rl *EnhancedRateLimiter) GetStats(ctx context.Context, key string) (map[string]interface{}, error) {
	bucketKey := fmt.Sprintf("%s:bucket:%s", rl.config.RedisPrefix, key)

	value, err := rl.redis.Get(ctx, bucketKey).Result()
	if err != nil && err != redis.Nil {
		return nil, err
	}

	stats := map[string]interface{}{
		"key":    key,
		"exists": err != redis.Nil,
	}

	if err == nil && value != "" {
		// Parse and return bucket statistics
		stats["value"] = value
	}

	return stats, nil
}

// GetConfig returns the current configuration
func (rl *EnhancedRateLimiter) GetConfig() EnhancedRateLimitConfig {
	rl.mu.RLock()
	defer rl.mu.RUnlock()
	return rl.config
}

// Reset resets rate limiting for a specific key
func (rl *EnhancedRateLimiter) Reset(ctx context.Context, key string) error {
	bucketKey := fmt.Sprintf("%s:bucket:%s", rl.config.RedisPrefix, key)
	return rl.redis.Del(ctx, bucketKey).Err()
}

// Helper function to get minimum of two integers
func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

// Helper function to get user tier from context or default
func GetUserTierFromContext(ctx context.Context, defaultTier RateLimitTier) RateLimitTier {
	// This would typically come from user authentication/authorization
	// For now, return the default tier
	return defaultTier
}
