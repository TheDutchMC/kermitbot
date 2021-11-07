# KermitBot
[![Build Status](https://drone.k8s.array21.dev/api/badges/TheDutchMC/kermitbot/status.svg)](https://drone.k8s.array21.dev/TheDutchMC/kermitbot)

Discord bot for the KermitCraft Discord Server. This bot is written in Rust using Serenity. It also serves certain data via a HTTP server using Actix Web. The application comes with a frontend which displays the data provided by the API

## Usage
1. Install docker & Docker compose
2. Set up a MySQL database
3. Create a [discord bot](https://discord.com/developers)
4. Use the following `docker-compose.yml`
```yaml
version: '3'
services:
    kermitbot:
        image: docker-registry.k8s.array21.dev/kermitbot:latest
        container_name: kermitbot
        environment:
        - 'BOT_TOKEN=YOUR_BOT_TOKEN'
        - 'GUILD_ID=The ID of the guild'
        - 'MYSQL_HOST=the_host'
        - 'MYSQL_DATABASE=the_database'
        - 'MYSQL_USERNAME=the_username'
        - 'MYSQL_PASSWORD=the_password'
        ports:
        - '8080:8080'
```
The API will be available at `/api/v1/`, the GUI at `/static/index.html`

### Supported tags
Every git tag is associated with the same Docker tag. E.g git tag `0.1.0` will have Docker tag `0.1.0`. Besides this, `latest` is also supported