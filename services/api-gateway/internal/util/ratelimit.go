package util

import (
	"context"
	"fmt"
	"strconv"
	"time"

	"github.com/redis/go-redis/v9"
)

type RateLimiter struct {
	redis *redis.Client
}

func NewRateLimiter(redis *redis.Client) *RateLimiter {
	return &RateLimiter{redis: redis}
}

func (rl *RateLimiter) Allow(ctx context.Context, key string, limit int, window time.Duration) (bool, error) {
	now := time.Now().Unix()
	windowStart := now - int64(window.Seconds())

	pipe := rl.redis.Pipeline()

	pipe.ZRemRangeByScore(ctx, key, "0", strconv.FormatInt(windowStart, 10))
	pipe.ZCard(ctx, key)
	pipe.ZAdd(ctx, key, redis.Z{Score: float64(now), Member: now})
	pipe.Expire(ctx, key, window)

	results, err := pipe.Exec(ctx)
	if err != nil {
		return false, fmt.Errorf("failed to execute rate limit pipeline: %w", err)
	}

	count := results[1].(*redis.IntCmd).Val()
	if count >= int64(limit) {
		return false, nil
	}

	return true, nil
}

func (rl *RateLimiter) GetCount(ctx context.Context, key string) (int64, error) {
	count, err := rl.redis.ZCard(ctx, key).Result()
	if err != nil {
		return 0, fmt.Errorf("failed to get rate limit count: %w", err)
	}
	return count, nil
}

func (rl *RateLimiter) Reset(ctx context.Context, key string) error {
	err := rl.redis.Del(ctx, key).Err()
	if err != nil {
		return fmt.Errorf("failed to reset rate limit: %w", err)
	}
	return nil
}
