service {
    name = "wemo-exporter"
    id   = "local"
    port = 3001
    address = "192.168.1.21"
    tags = [
        "metrics"
    ]
}