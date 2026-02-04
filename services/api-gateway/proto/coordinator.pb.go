package proto

import (
	"context"
	"time"
)

type SearchRequest struct {
	Query     string            `json:"query"`
	Indexes   []string          `json:"indexes"`
	Page      int32             `json:"page"`
	PageSize  int32             `json:"page_size"`
	Filters   map[string]string `json:"filters"`
	Fields    []string          `json:"fields"`
	Highlight bool              `json:"highlight"`
	SortBy    string            `json:"sort_by"`
	SortOrder string            `json:"sort_order"`
	Explain   bool              `json:"explain"`
}

type SearchResponse struct {
	Results    []*SearchResult `json:"results"`
	Total      int32          `json:"total"`
	Page       int32          `json:"page"`
	PageSize   int32          `json:"page_size"`
	TotalPages int32          `json:"total_pages"`
	TookMs     float64        `json:"took_ms"`
}

type SearchResult struct {
	Id         string            `json:"id"`
	Score      float64           `json:"score"`
	Fields     map[string]string `json:"fields"`
	Highlights map[string]string `json:"highlights"`
	Explain    map[string]float64 `json:"explain"`
}

type GetDocumentRequest struct {
	IndexId    string `json:"index_id"`
	DocumentId string `json:"document_id"`
}

type DocumentResponse struct {
	Id     string            `json:"id"`
	Fields map[string]string `json:"fields"`
	Score  float64           `json:"score"`
}

type AddDocumentRequest struct {
	IndexId string            `json:"index_id"`
	Fields  map[string]string `json:"fields"`
}

type AddDocumentResponse struct {
	Id      string `json:"id"`
	Success bool   `json:"success"`
	Message string `json:"message"`
}

type UpdateDocumentRequest struct {
	IndexId    string            `json:"index_id"`
	DocumentId string            `json:"document_id"`
	Fields     map[string]string `json:"fields"`
}

type UpdateDocumentResponse struct {
	Success bool   `json:"success"`
	Message string `json:"message"`
}

type DeleteDocumentRequest struct {
	IndexId    string `json:"index_id"`
	DocumentId string `json:"document_id"`
}

type DeleteDocumentResponse struct {
	Success bool   `json:"success"`
	Message string `json:"message"`
}

type BatchDocumentsRequest struct {
	IndexId   string              `json:"index_id"`
	Documents []map[string]string `json:"documents"`
	Refresh   bool                `json:"refresh"`
}

type BatchDocumentsResponse struct {
	SuccessCount int32    `json:"success_count"`
	FailureCount int32    `json:"failure_count"`
	Errors       []string `json:"errors"`
}

type CreateIndexRequest struct {
	Name      string            `json:"name"`
	IndexType string            `json:"index_type"`
	Fields    []string          `json:"fields"`
	Options   map[string]string `json:"options"`
}

type CreateIndexResponse struct {
	Id      string `json:"id"`
	Success bool   `json:"success"`
	Message string `json:"message"`
}

type ListIndexesRequest struct {
	Page     int32 `json:"page"`
	PageSize int32 `json:"page_size"`
}

type ListIndexesResponse struct {
	Indexes []*IndexInfo `json:"indexes"`
	Total   int32        `json:"total"`
}

type IndexInfo struct {
	Id            string `json:"id"`
	Name          string `json:"name"`
	IndexType     string `json:"index_type"`
	DocumentCount int64  `json:"document_count"`
	SizeBytes     int64  `json:"size_bytes"`
	CreatedAt     string `json:"created_at"`
	UpdatedAt     string `json:"updated_at"`
}

type GetIndexRequest struct {
	IndexId string `json:"index_id"`
}

type GetIndexResponse struct {
	Index *IndexInfo `json:"index"`
}

type DeleteIndexRequest struct {
	IndexId string `json:"index_id"`
}

type DeleteIndexResponse struct {
	Success bool   `json:"success"`
	Message string `json:"message"`
}

type RebuildIndexRequest struct {
	IndexId string `json:"index_id"`
	Async   bool   `json:"async"`
}

type RebuildIndexResponse struct {
	Success bool   `json:"success"`
	Message string `json:"message"`
	TaskId  string `json:"task_id"`
}

type HealthCheckRequest struct {
	Service string `json:"service"`
}

type HealthCheckResponse struct {
	Status        string            `json:"status"`
	Version       string            `json:"version"`
	UptimeSeconds int64             `json:"uptime_seconds"`
	Details       map[string]string `json:"details"`
}

type ServiceStatus struct {
	Name       string `json:"name"`
	Address    string `json:"address"`
	Status     string `json:"status"`
	LatencyMs  int64  `json:"latency_ms"`
	LastCheck  string `json:"last_check"`
	Message    string `json:"message"`
}

type SearchServiceClient interface {
	Search(ctx context.Context, in *SearchRequest, opts ...interface{}) (*SearchResponse, error)
}

type DocumentServiceClient interface {
	GetDocument(ctx context.Context, in *GetDocumentRequest, opts ...interface{}) (*DocumentResponse, error)
	AddDocument(ctx context.Context, in *AddDocumentRequest, opts ...interface{}) (*AddDocumentResponse, error)
	UpdateDocument(ctx context.Context, in *UpdateDocumentRequest, opts ...interface{}) (*UpdateDocumentResponse, error)
	DeleteDocument(ctx context.Context, in *DeleteDocumentRequest, opts ...interface{}) (*DeleteDocumentResponse, error)
	BatchDocuments(ctx context.Context, in *BatchDocumentsRequest, opts ...interface{}) (*BatchDocumentsResponse, error)
}

type IndexServiceClient interface {
	CreateIndex(ctx context.Context, in *CreateIndexRequest, opts ...interface{}) (*CreateIndexResponse, error)
	ListIndexes(ctx context.Context, in *ListIndexesRequest, opts ...interface{}) (*ListIndexesResponse, error)
	GetIndex(ctx context.Context, in *GetIndexRequest, opts ...interface{}) (*GetIndexResponse, error)
	DeleteIndex(ctx context.Context, in *DeleteIndexRequest, opts ...interface{}) (*DeleteIndexResponse, error)
	RebuildIndex(ctx context.Context, in *RebuildIndexRequest, opts ...interface{}) (*RebuildIndexResponse, error)
}

type HealthClient interface {
	Check(ctx context.Context, in *HealthCheckRequest, opts ...interface{}) (*HealthCheckResponse, error)
}

type UnimplementedSearchServiceServer struct{}

func (UnimplementedSearchServiceServer) Search(ctx context.Context, req *SearchRequest) (*SearchResponse, error) {
	return nil, nil
}

type UnimplementedDocumentServiceServer struct{}

func (UnimplementedDocumentServiceServer) GetDocument(ctx context.Context, req *GetDocumentRequest) (*DocumentResponse, error) {
	return nil, nil
}

func (UnimplementedDocumentServiceServer) AddDocument(ctx context.Context, req *AddDocumentRequest) (*AddDocumentResponse, error) {
	return nil, nil
}

func (UnimplementedDocumentServiceServer) UpdateDocument(ctx context.Context, req *UpdateDocumentRequest) (*UpdateDocumentResponse, error) {
	return nil, nil
}

func (UnimplementedDocumentServiceServer) DeleteDocument(ctx context.Context, req *DeleteDocumentRequest) (*DeleteDocumentResponse, error) {
	return nil, nil
}

func (UnimplementedDocumentServiceServer) BatchDocuments(ctx context.Context, req *BatchDocumentsRequest) (*BatchDocumentsResponse, error) {
	return nil, nil
}

type UnimplementedIndexServiceServer struct{}

func (UnimplementedIndexServiceServer) CreateIndex(ctx context.Context, req *CreateIndexRequest) (*CreateIndexResponse, error) {
	return nil, nil
}

func (UnimplementedIndexServiceServer) ListIndexes(ctx context.Context, req *ListIndexesRequest) (*ListIndexesResponse, error) {
	return nil, nil
}

func (UnimplementedIndexServiceServer) GetIndex(ctx context.Context, req *GetIndexRequest) (*GetIndexResponse, error) {
	return nil, nil
}

func (UnimplementedIndexServiceServer) DeleteIndex(ctx context.Context, req *DeleteIndexRequest) (*DeleteIndexResponse, error) {
	return nil, nil
}

func (UnimplementedIndexServiceServer) RebuildIndex(ctx context.Context, req *RebuildIndexRequest) (*RebuildIndexResponse, error) {
	return nil, nil
}

type UnimplementedHealthServer struct{}

func (UnimplementedHealthServer) Check(ctx context.Context, req *HealthCheckRequest) (*HealthCheckResponse, error) {
	return nil, nil
}