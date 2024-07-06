# Configuration

- Configure `Config.toml`.
    - Configure your Up secret.
    - Configure your Notion secret.
    - Create a database in Notion and copy its id.

# Running

- Follow the Up API section.
- Open in RustRover and execute.

# Up API

```bash
brew install openapi-generator
openapi-generator generate -i up.json -g rust -o ./upapi
```

# Notion API
Notion does not publish an OpenAPI spec. There is one in this repo, but I have committed the generated code because I needed to fix some issues:

- The generated code does not have any models. I have started creating some in `mod.rs`.
- The reqwest `.json` function does not build a request body if the input is a string, which it is in the generated code. I have fixed this by using my own models.

