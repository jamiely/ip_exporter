Intro
=====

Monitors the IPs that are bound to the machine. Returns
Prometheus metrics with labels indicating whether the
pre-configured set of IPs have been bound. Set a
space-delimited list of IPs in the `IP_LIST` environment
variable.

This is one solution for monitoring Virtual IP-based
solutions where IPs might float around multiple hosts.

Running
=======

```bash
rustup override set nightly
rustup update
export IP_LIST="127.0.0.1 192.168.0.3 192.168.0.2"
cargo run
```

A Docker image is available.

```bash
docker run -e IP_LIST="127.0.0.1 192.168.0.3 192.168.0.2" \
  -p 8000:8000 jamiely/ip_exporter
```

Example Response
================

```
% docker run -e IP_LIST="127.0.0.1 192.168.0.3 192.168.0.2" \
  -p 8000:8000 jamiely/ip_exporter
% curl localhost:8000/metrics

# HELP ip_is_bound Whether or not the IP is bound. 1 is the IP is bound and 0 if not.
# TYPE ip_is_bound gauge
ip_is_bound{ip="127.0.0.1"} 1
ip_is_bound{ip="192.168.0.3"} 0
ip_is_bound{ip="192.168.0.2"} 0
```

