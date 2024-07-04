# Frontend Gateway

A Rust Frontend Gateway built on top of custom gateway library.

[Crate API Documentation](https://majksa-dev.github.io/frontend-gateway/)

**Table of Contents**

- [Running](#running)
- [Gateway Configuration](#gateway-configuration)
- [Configuration file reference](#configuration-file-reference)
- [Example configuration](#example-configuration)

## Running

<!-- x-release-please-start-version -->

`docker run --rm -p 80:80 ghcr.io/majksa-dev/frontend-gateway:0.1.1`

<!-- x-release-please-end -->

## Gateway Configuration

| **ENV**          | **Description**                                              | **Default** |
| ---------------- | ------------------------------------------------------------ | ----------- |
| CONFIG_FILE      | Path to the configuration file                               |             |
| REDIS_CACHE_URL  | Connection URL for redis database                            |             |
| HOST             | HTTP host that the gateway will be exposed on.               | 127.0.0.1   |
| PORT             | HTTP port that the gateway will be exposed on.               | 80          |
| HEALTHCHECK_PORT | HTTP port that gateway healthcheck endpoint is available on. | 9000        |

## Configuration file reference

Json schema is available at: [./config.schema.json](https://raw.githubusercontent.com/majksa-dev/server-gateway/main/config.schema.json)

## Example configuration

```json
{
  "$schema": "https://raw.githubusercontent.com/majksa-dev/frontend-gateway/main/config.schema.json",
  "apps": {
    "app": {
      "upstream": {
        "host": "localhost",
        "port": 3005
      },
      "auth": [
        {
          "token": "hello",
          "quota": {
            "total": {
              "amount": 1000,
              "interval": {
                "amount": 1,
                "unit": "minutes"
              }
            },
            "user": {
              "amount": 10,
              "interval": {
                "amount": 1,
                "unit": "minutes"
              }
            }
          }
        }
      ],
      "endpoints": [
        {
          "path": "/api/hello",
          "id": "hello",
          "method": "GET",
          "quota": {
            "amount": 100,
            "interval": {
              "amount": 1,
              "unit": "minutes"
            }
          }
        },
        {
          "path": "/api/update/:id",
          "id": "update",
          "method": "POST",
          "quota": {
            "amount": 1,
            "interval": {
              "amount": 1,
              "unit": "minutes"
            }
          }
        }
      ]
    }
  }
}
```
