job "wemo-exporter" {
  datacenters = ["home"]
  type = "service"

  group "exporter" {
    count = 1

    network {
      mode = "bridge"
      port "metrics" {
        to = 3001
      }
    }
    service {
      name = "wemo-exporter"
      tags = ["metrics"]
      port = "metrics"
    }

    restart {
      # The number of attempts to run the job within the specified interval.
      attempts = 2
      interval = "30m"

      # The "delay" parameter specifies the duration to wait before restarting
      # a task after it has failed.
      delay = "15s"

      # The "mode" parameter controls what happens when a task has restarted
      # "attempts" times within the interval. "delay" mode delays the next
      # restart until the next interval. "fail" mode does not restart the task
      # if "attempts" has been hit within the interval.
      mode = "fail"
    }
    
    task "exporter" {
      driver = "exec"
      artifact {
        source = "https://github.com/7imbrook/wemo-exporter/releases/download/dev/wemo-exporter-aarch64-unknown-linux-gnu.zip"
      }
      config {
        command = "wemo-exporter" 
        args = ["-s"]
      }
      template {
        data = file("./settings.yaml")
        destination = "local/settings.yaml"
      }
      resources {
        cpu    = 500 
        memory = 256 
      }
    }
  }
}