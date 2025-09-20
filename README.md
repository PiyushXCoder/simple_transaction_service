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
