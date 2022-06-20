package config


// Logical App representation for proxy
type Application struct {
    Name string
    Ports []int
    Origins []string
}

type Proxy struct {
    Apps []Application
}
