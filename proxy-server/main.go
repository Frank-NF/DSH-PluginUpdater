package main

import (
	"context"
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
	"net/url"
	"os"
	"regexp"
	"strings"
	"time"

	"github.com/gin-contrib/cors"
	"github.com/gin-gonic/gin"
	"github.com/go-redis/redis/v8"
	"github.com/joho/godotenv"
)

// ============ 配置 ============

type Config struct {
	Port         string
	GitHubToken  string
	ProxyToken   string
	RedisAddr    string
	RedisPassword string
	RedisDB      int
	CacheTTL     time.Duration
	AllowedRepos []string
}

var config Config
var redisClient *redis.Client
var ctx = context.Background()

// ============ GitHub API 响应结构 ============

type GitHubRelease struct {
	TagName     string        `json:"tag_name"`
	Name        string        `json:"name"`
	HTMLURL     string        `json:"html_url"`
	Body        string        `json:"body"`
	Assets      []GitHubAsset `json:"assets"`
	PublishedAt string        `json:"published_at"`
}

type GitHubAsset struct {
	Name               string `json:"name"`
	BrowserDownloadURL string `json:"browser_download_url"`
	Size               int64  `json:"size"`
	ContentType        string `json:"content_type"`
}

// ============ 主函数 ============

func main() {
	loadConfig()
	initRedis()

	r := gin.Default()

	// CORS 配置
	r.Use(cors.New(cors.Config{
		AllowOrigins:     []string{"*"},
		AllowMethods:     []string{"GET", "POST", "OPTIONS"},
		AllowHeaders:     []string{"Origin", "Content-Type", "X-Proxy-Token"},
		ExposeHeaders:    []string{"Content-Length", "Content-Disposition"},
		AllowCredentials: true,
		MaxAge:           12 * time.Hour,
	}))

	// 健康检查
	r.GET("/health", healthCheck)

	// API 路由组
	api := r.Group("/api")
	api.Use(authMiddleware())
	{
		api.GET("/github/latest", getLatestRelease)
		api.GET("/github/releases", getReleases)
		api.GET("/github/download", downloadAsset)
		api.GET("/plugins/list", getPluginList)
	}

	log.Printf("DSH Plugin Proxy Server starting on :%s", config.Port)
	if err := r.Run(":" + config.Port); err != nil {
		log.Fatalf("Failed to start server: %v", err)
	}
}

// ============ 配置加载 ============

func loadConfig() {
	_ = godotenv.Load()

	config.Port = getEnv("PORT", "8080")
	config.GitHubToken = getEnv("GITHUB_TOKEN", "")
	config.ProxyToken = getEnv("PROXY_TOKEN", "")
	config.RedisAddr = getEnv("REDIS_ADDR", "localhost:6379")
	config.RedisPassword = getEnv("REDIS_PASSWORD", "")
	config.RedisDB = getEnvInt("REDIS_DB", 0)
	config.CacheTTL = time.Duration(getEnvInt("CACHE_TTL_MINUTES", 30)) * time.Minute

	allowedRepos := getEnv("ALLOWED_REPOS", "")
	if allowedRepos != "" {
		config.AllowedRepos = strings.Split(allowedRepos, ",")
	}

	log.Println("Configuration loaded")
}

func getEnv(key, defaultValue string) string {
	if value := os.Getenv(key); value != "" {
		return value
	}
	return defaultValue
}

func getEnvInt(key string, defaultValue int) int {
	if value := os.Getenv(key); value != "" {
		var intValue int
		if _, err := fmt.Sscanf(value, "%d", &intValue); err == nil {
			return intValue
		}
	}
	return defaultValue
}

// ============ Redis 初始化 ============

func initRedis() {
	redisClient = redis.NewClient(&redis.Options{
		Addr:     config.RedisAddr,
		Password: config.RedisPassword,
		DB:       config.RedisDB,
	})

	_, err := redisClient.Ping(ctx).Result()
	if err != nil {
		log.Printf("Warning: Redis connection failed, caching disabled: %v", err)
		redisClient = nil
	} else {
		log.Println("Redis connected successfully")
	}
}

// ============ 中间件 ============

func authMiddleware() gin.HandlerFunc {
	return func(c *gin.Context) {
		// 如果配置了代理 token，则验证
		if config.ProxyToken != "" {
			token := c.GetHeader("X-Proxy-Token")
			if token != config.ProxyToken {
				c.JSON(http.StatusUnauthorized, gin.H{
					"error": "Unauthorized: invalid proxy token",
				})
				c.Abort()
				return
			}
		}
		c.Next()
	}
}

// ============ 健康检查 ============

func healthCheck(c *gin.Context) {
	c.JSON(http.StatusOK, gin.H{
		"status":  "ok",
		"service": "DSH Plugin Proxy",
		"version": "1.0.0",
		"time":    time.Now().Format(time.RFC3339),
	})
}

// ============ GitHub API 调用 ============

func getLatestRelease(c *gin.Context) {
	repo := c.Query("repo")
	if err := validateRepo(repo); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	// 检查缓存
	cacheKey := fmt.Sprintf("release:latest:%s", repo)
	if cached := getCache(cacheKey); cached != "" {
		var release GitHubRelease
		if json.Unmarshal([]byte(cached), &release) == nil {
			c.JSON(http.StatusOK, release)
			return
		}
	}

	// 请求 GitHub API
	apiURL := fmt.Sprintf("https://api.github.com/repos/%s/releases/latest", repo)
	body, err := fetchGitHubAPI(apiURL)
	if err != nil {
		c.JSON(http.StatusBadGateway, gin.H{"error": fmt.Sprintf("GitHub API error: %v", err)})
		return
	}

	var release GitHubRelease
	if err := json.Unmarshal(body, &release); err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to parse GitHub response"})
		return
	}

	// 写入缓存
	setCache(cacheKey, string(body))

	c.JSON(http.StatusOK, release)
}

func getReleases(c *gin.Context) {
	repo := c.Query("repo")
	perPage := c.DefaultQuery("per_page", "10")

	if err := validateRepo(repo); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	apiURL := fmt.Sprintf("https://api.github.com/repos/%s/releases?per_page=%s", repo, perPage)
	body, err := fetchGitHubAPI(apiURL)
	if err != nil {
		c.JSON(http.StatusBadGateway, gin.H{"error": fmt.Sprintf("GitHub API error: %v", err)})
		return
	}

	var releases []GitHubRelease
	if err := json.Unmarshal(body, &releases); err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to parse GitHub response"})
		return
	}

	c.JSON(http.StatusOK, releases)
}

func downloadAsset(c *gin.Context) {
	repo := c.Query("repo")
	tag := c.Query("tag")
	assetName := c.Query("asset")

	if err := validateRepo(repo); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	if tag == "" || assetName == "" {
		c.JSON(http.StatusBadRequest, gin.H{"error": "tag and asset parameters are required"})
		return
	}

	// 构建下载 URL
	downloadURL := fmt.Sprintf("https://github.com/%s/releases/download/%s/%s", repo, tag, assetName)

	// 创建请求
	req, err := http.NewRequest("GET", downloadURL, nil)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to create request"})
		return
	}

	if config.GitHubToken != "" {
		req.Header.Set("Authorization", "token "+config.GitHubToken)
	}

	client := &http.Client{
		Timeout: 300 * time.Second,
		CheckRedirect: func(req *http.Request, via []*http.Request) error {
			return nil
		},
	}

	resp, err := client.Do(req)
	if err != nil {
		c.JSON(http.StatusBadGateway, gin.H{"error": fmt.Sprintf("Download failed: %v", err)})
		return
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		c.JSON(resp.StatusCode, gin.H{"error": fmt.Sprintf("GitHub returned status: %d", resp.StatusCode)})
		return
	}

	// 设置响应头
	c.Header("Content-Disposition", fmt.Sprintf("attachment; filename=\"%s\"", assetName))
	c.Header("Content-Type", resp.Header.Get("Content-Type"))
	c.Header("Content-Length", fmt.Sprintf("%d", resp.ContentLength))

	// 流式转发
	c.Status(http.StatusOK)
	_, _ = io.Copy(c.Writer, resp.Body)
}

// ============ 插件列表（官网同步用） ============

func getPluginList(c *gin.Context) {
	// 从缓存或配置中获取插件列表
	cacheKey := "plugins:list"
	if cached := getCache(cacheKey); cached != "" {
		c.Data(http.StatusOK, "application/json", []byte(cached))
		return
	}

	// 默认插件列表（可从配置文件或数据库加载）
	plugins := []map[string]interface{}{
		{
			"id":          "dsh-agent-core",
			"name":        "DSH Agent 本体",
			"description": "DSH Agent 核心程序，提供基础运行环境与能力调度",
			"github_repo": "DSH-Team/DSH-Agent",
			"type":        "agent-core",
		},
	}

	// 这里可以扩展：从配置文件加载更多插件
	// 或者从 GitHub Organization 自动扫描

	data, _ := json.Marshal(plugins)
	setCache(cacheKey, string(data))

	c.Data(http.StatusOK, "application/json", data)
}

// ============ 工具函数 ============

func validateRepo(repo string) error {
	if repo == "" {
		return fmt.Errorf("repo parameter is required")
	}

	// 格式验证: owner/repo
	matched, _ := regexp.MatchString(`^[a-zA-Z0-9][a-zA-Z0-9-]*\/[a-zA-Z0-9._-]+$`, repo)
	if !matched {
		return fmt.Errorf("invalid repo format, expected owner/repo")
	}

	// 白名单验证（如果配置了）
	if len(config.AllowedRepos) > 0 {
		allowed := false
		for _, allowedRepo := range config.AllowedRepos {
			if strings.EqualFold(allowedRepo, repo) {
				allowed = true
				break
			}
		}
		if !allowed {
			return fmt.Errorf("repo not in allowed list")
		}
	}

	return nil
}

func fetchGitHubAPI(apiURL string) ([]byte, error) {
	req, err := http.NewRequest("GET", apiURL, nil)
	if err != nil {
		return nil, err
	}

	req.Header.Set("Accept", "application/vnd.github.v3+json")
	if config.GitHubToken != "" {
		req.Header.Set("Authorization", "token "+config.GitHubToken)
	}

	client := &http.Client{Timeout: 30 * time.Second}
	resp, err := client.Do(req)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		body, _ := io.ReadAll(resp.Body)
		return nil, fmt.Errorf("GitHub API returned %d: %s", resp.StatusCode, string(body))
	}

	return io.ReadAll(resp.Body)
}

func getCache(key string) string {
	if redisClient == nil {
		return ""
	}
	val, err := redisClient.Get(ctx, key).Result()
	if err != nil {
		return ""
	}
	return val
}

func setCache(key, value string) {
	if redisClient == nil {
		return
	}
	_ = redisClient.Set(ctx, key, value, config.CacheTTL).Err()
}

// URL 编码辅助
func urlEncode(s string) string {
	return url.QueryEscape(s)
}
