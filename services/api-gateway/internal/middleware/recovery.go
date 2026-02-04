package middleware

import (
    "fmt"
    "net/http"
    "runtime/debug"

    "github.com/gin-gonic/gin"
    "github.com/flexsearch/api-gateway/internal/util"
)

func RecoveryMiddleware() gin.HandlerFunc {
    return func(c *gin.Context) {
        defer func() {
            if err := recover(); err != nil {
                stack := debug.Stack()
                gin.DefaultWriter.Write([]byte(fmt.Sprintf("Panic recovered: %v\n%s", err, stack)))

                c.JSON(http.StatusInternalServerError, gin.H{
                    "error": "Internal server error",
                    "details": "An unexpected error occurred",
                })
                c.Abort()
            }
        }()

        c.Next()
    }
}

func ErrorHandlerMiddleware() gin.HandlerFunc {
    return func(c *gin.Context) {
        c.Next()

        if len(c.Errors) > 0 {
            err := c.Errors.Last().Err

            if appErr, ok := err.(*util.AppError); ok {
                c.JSON(appErr.Code, gin.H{
                    "error": appErr.Message,
                    "details": appErr.Details,
                })
            } else {
                c.JSON(http.StatusInternalServerError, gin.H{
                    "error": "Internal server error",
                    "details": err.Error(),
                })
            }
        }
    }
}
