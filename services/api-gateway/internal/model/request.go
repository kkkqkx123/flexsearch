package model

type SearchRequest struct {
    Query   string                 `json:"query" binding:"required,min=1,max=100"`
    Limit   int                    `json:"limit" binding:"omitempty,min=1,max=100"`
    Offset  int                    `json:"offset" binding:"omitempty,min=0"`
    Filters map[string]string        `json:"filters"`
    Options map[string]interface{}   `json:"options"`
}

type SearchResponse struct {
    Results []SearchResult `json:"results"`
    Total   int          `json:"total"`
    Latency int          `json:"latency"`
    Engine  string       `json:"engine"`
}

type SearchResult struct {
    ID       string                 `json:"id"`
    Score    float64                `json:"score"`
    Document map[string]interface{}  `json:"document"`
    Highlight map[string]string      `json:"highlight,omitempty"`
}

type CreateDocumentRequest struct {
    ID      string                 `json:"id" binding:"required"`
    Title   string                 `json:"title" binding:"required,min=1,max=200"`
    Content string                 `json:"content" binding:"required,min=1"`
    Fields  map[string]interface{}  `json:"fields"`
}

type UpdateDocumentRequest struct {
    Title   string                 `json:"title" binding:"omitempty,min=1,max=200"`
    Content string                 `json:"content" binding:"omitempty,min=1"`
    Fields  map[string]interface{}  `json:"fields"`
}

type DocumentResponse struct {
    ID      string                 `json:"id"`
    Title   string                 `json:"title"`
    Content string                 `json:"content"`
    Fields  map[string]interface{}  `json:"fields"`
    Created int64                 `json:"created"`
    Updated int64                 `json:"updated"`
}

type BatchDocumentRequest struct {
    Documents []CreateDocumentRequest `json:"documents" binding:"required,min=1,max=100"`
}

type BatchDocumentResponse struct {
    Success []string `json:"success"`
    Failed  []string `json:"failed"`
}

type CreateIndexRequest struct {
    ID      string                 `json:"id" binding:"required"`
    Name    string                 `json:"name" binding:"required,min=1,max=100"`
    Fields  []IndexField           `json:"fields" binding:"required,min=1"`
    Options map[string]interface{}  `json:"options"`
}

type IndexField struct {
    Name     string `json:"name" binding:"required"`
    Type     string `json:"type" binding:"required,oneof=text keyword number date"`
    Indexed  bool   `json:"indexed"`
    Stored   bool   `json:"stored"`
}

type IndexResponse struct {
    ID      string                 `json:"id"`
    Name    string                 `json:"name"`
    Fields  []IndexField           `json:"fields"`
    Options map[string]interface{}  `json:"options"`
    Status  string                 `json:"status"`
    Created int64                 `json:"created"`
}

type ErrorResponse struct {
    Error   string `json:"error"`
    Details string `json:"details,omitempty"`
    Code    int    `json:"code"`
}

type SuccessResponse struct {
    Message string `json:"message"`
    Data    interface{} `json:"data,omitempty"`
}
