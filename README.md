## Simple Transaction Service

<br/>
<p align="center">
<img width="300" height="" alt="Image" src="./transaction.png" align="center"/>
</p>

[Api Documentatin](./API-Documentation.md)

[Design Documentation](DESIGN.md)

## How to run??

1. Prerequisite
   - docker

2. Start Docker

   ```bash
   docker-compose up
   ```

3. Migrate database

   ```bash
   docker-compose exec -it app ./cli -m
   ```

4. Add API Key

   ```bash
   docker-compose exec -it app ./cli -a
   ```

After running it open [Api Documentatin](./API-Documentation.md) and check how to use

## Open Telemetry

Open Telemetry is included! to check for stats run following command to start jaeger

```bash
docker run -d --name jaeger \
  -e COLLECTOR_ZIPKIN_HOST_PORT=:9411 \
  -e COLLECTOR_OTLP_ENABLED=true \
  -p 6831:6831/udp \
  -p 6832:6832/udp \
  -p 5778:5778 \
  -p 16686:16686 \
  -p 4317:4317 \
  -p 4318:4318 \
  -p 14250:14250 \
  -p 14268:14268 \
  -p 14269:14269 \
  -p 9411:9411 \
  jaegertracing/all-in-one:latest
```

Then visit `http://localhost:16686/`
