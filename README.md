# About

Extract insights from your [Up](https://up.com.au/) account.

# Features
- Import Up spending transactions into a Notion database.

# Running
- Configure `Config.toml`.
  - Configure your [Up secret](https://developer.up.com.au/).
  - Configure your [Notion secret](https://developers.notion.com/docs/authorization).
  - Create a database in Notion and copy its id from the URL.
- Follow the Up API section.
- Open in RustRover and execute.

# Up API

```bash
brew install openapi-generator
openapi-generator generate -i up.json -g rust -o ./upapi
```

# Notion API
Notion does not publish an OpenAPI spec, but they do publish a postman collection. The Notion OpenAPI spec in this repo is generated from the postman collection. However, when generating the client, there are some issues. Therefore, I have committed the generated code with fixes:

- The generated code does not have any models. I have started creating some in `notionapi/src/models/mod.rs`.
- The reqwest `.json` function does not build a request body if the input is a string, which it is in the generated code. I have started fixing this by updating the client to serialise my own models.

