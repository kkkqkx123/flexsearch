package main

import (
	"context"
	"fmt"
	"log"
	"net/http"
	"os"
	"os/signal"
	"syscall"
	"time"

	"github.com/flexsearch/api-gateway/internal/config"
	"github.com/flexsearch/api-gateway/internal/handler"
	"github.com/flexsearch/api-gateway/internal/middleware"
	"github.com/flexsearch/api-gateway/internal/util"
	"github.com/gin-gonic/gin"
	"github.com/redis/go-redis/v9"
)

func main() {
	cfg, err := config.Load()
	if err != nil {
		log.Fatalf("Failed to load config: %v", err)
	}

	if cfg.Server.Mode == "release" {
		gin.SetMode(gin.ReleaseMode)
	}

	redisClient := redis.NewClient(&redis.Options{
		Addr:     fmt.Sprintf("%s:%d", cfg.Redis.Host, cfg.Redis.Port),
		Password: cfg.Redis.Password,
		DB:       cfg.Redis.DB,
	})
	defer redisClient.Close()

	jwtManager := util.NewJWTManager(cfg.JWT.Secret, cfg.JWT.Issuer, cfg.JWT.Expiration)
	rateLimiter := util.NewRateLimiter(redisClient)

	router := gin.New()

	router.Use(middleware.Logger())
	router.Use(middleware.RecoveryMiddleware())
	router.Use(middleware.ErrorHandlerMiddleware())

	if cfg.CORS.Enabled {
		router.Use(middleware.CORSMiddleware(middleware.CORSConfig{
			AllowOrigins:     cfg.CORS.AllowOrigins,
			AllowMethods:     cfg.CORS.AllowMethods,
			AllowHeaders:     cfg.CORS.AllowHeaders,
			AllowCredentials: cfg.CORS.AllowCredentials,
		}))
	}

	if cfg.RateLimit.Enabled {
		router.Use(middleware.RateLimitMiddleware(rateLimiter, middleware.RateLimitConfig{
			Limit:  cfg.RateLimit.DefaultLimit,
			Window: time.Minute,
			ByUser: cfg.RateLimit.ByUser,
			ByIP:   cfg.RateLimit.ByIP,
		}))
	}

	config.SetupRoutes(router, cfg, jwtManager)

	healthHandler := handler.NewHealthHandler()
	router.GET("/health", healthHandler.Check)
	router.GET("/health/services", healthHandler.CheckServices)

	srv := &http.Server{
		Addr:           fmt.Sprintf(":%d", cfg.Server.Port),
		Handler:        router,
		ReadTimeout:    time.Duration(cfg.Server.ReadTimeout) * time.Second,
		WriteTimeout:   time.Duration(cfg.Server.WriteTimeout) * time.Second,
		MaxHeaderBytes: 1 << 20,
	}

	go func() {
		log.Printf("Starting server on port %d", cfg.Server.Port)
		if err := srv.ListenAndServe(); err != nil && err != http.ErrServerClosed {
			log.Fatalf("Failed to start server: %v", err)
		}
	}()

	quit := make(chan os.Signal, 1)
	signal.Notify(quit, syscall.SIGINT, syscall.SIGTERM)
	<-quit

	log.Println("Shutting down server...")

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	if err := srv.Shutdown(ctx); err != nil {
		log.Fatalf("Server forced to shutdown: %v", err)
	}

	log.Println("Server exited")
}
