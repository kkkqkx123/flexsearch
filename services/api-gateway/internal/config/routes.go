package config

import (
	"github.com/gin-gonic/gin"
	"github.com/flexsearch/api-gateway/internal/handler"
	"github.com/flexsearch/api-gateway/internal/middleware"
	"github.com/flexsearch/api-gateway/internal/util"
)

func SetupRoutes(router *gin.Engine, cfg *Config, jwtManager *util.JWTManager) {
	v1 := router.Group("/api/v1")
	{
		searchHandler := handler.NewSearchHandler()
		v1.GET("/search", middleware.OptionalAuthMiddleware(jwtManager), searchHandler.Search)
		v1.POST("/search", middleware.OptionalAuthMiddleware(jwtManager), searchHandler.Search)

		documentHandler := handler.NewDocumentHandler()
		v1.POST("/documents", middleware.AuthMiddleware(jwtManager), documentHandler.Create)
		v1.GET("/documents/:id", middleware.AuthMiddleware(jwtManager), documentHandler.Get)
		v1.PUT("/documents/:id", middleware.AuthMiddleware(jwtManager), documentHandler.Update)
		v1.DELETE("/documents/:id", middleware.AuthMiddleware(jwtManager), documentHandler.Delete)
		v1.POST("/documents/batch", middleware.AuthMiddleware(jwtManager), documentHandler.Batch)

		indexHandler := handler.NewIndexHandler()
		v1.POST("/indexes", middleware.AuthMiddleware(jwtManager), indexHandler.Create)
		v1.GET("/indexes", middleware.AuthMiddleware(jwtManager), indexHandler.List)
		v1.GET("/indexes/:id", middleware.AuthMiddleware(jwtManager), indexHandler.Get)
		v1.DELETE("/indexes/:id", middleware.AuthMiddleware(jwtManager), indexHandler.Delete)
		v1.POST("/indexes/:id/rebuild", middleware.AuthMiddleware(jwtManager), indexHandler.Rebuild)
	}
}
