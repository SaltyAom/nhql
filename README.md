# NhQL
Unofficial GraphQL Reverse Proxy Server for nHentai written in Rust.

## Feature
- High Performance and Low Memory Usage
- GraphQL and Playground
- One Command Deploy to Google Cloud

## Built with and powered by
- Rust
- Actix Web
- Juniper
- Pulumi
- TypeScript (Infra)
- Google Cloud

## Project Structure
- code
    - Where server code is located
- infra
    - Infrastructure of project for deployment

## Code Structure
- src
    - main.rs
        - Maikn server file, contains configuration for pluggable module.

    - models
        - Models for global usage

        - nhapi
            - Formatted structure of nHentai's data
        - nhentai
            - Raw nHentai's data

    - modules
        - Pluggable module for `main.rs` (Server Configuration)

        - graphql
            - Where GraphQL run
        - landing
            - To check if project is working, visit `/` 
        - proxy
            - Proxy module for requesting to nHentai API
    
    - services
        - Encapsulated business logic for global usage

        - schema
            - Structure of GraphQL

## Infra Structure
Same as starter template of Pulumi Google Cloud TypeScript Starter.

- index.ts
    - Infrastructure configuration

## Requirement
- [Google Cloud Account](https://cloud.google.com/)
- [Docker](https://www.docker.com/)
- [Pulumi](https://www.pulumi.com/)
- [Setup Pulumi to access Cloud Run](https://www.pulumi.com/docs/tutorials/gcp/gcp-ts-cloudrun/#prerequisites)
- [Setup Docker to access Google Cloud](https://www.pulumi.com/docs/tutorials/gcp/gcp-ts-docker-gcr-cloudrun/#prerequisites)
- [Nodejs](https://nodejs.org/)

## Gotcha
If you're deploying to dedicated project uncomment `enableCloudRun` and `dependsOn`
```typescript
    /* Uncomment this */
24  // const enableCloudRun = new gcp.projects.Service('EnableCloudRun', {
25  // 	service: 'run.googleapis.com'
26  // })

48     },
49  // { dependsOn: enableCloudRun }
50  }
```
