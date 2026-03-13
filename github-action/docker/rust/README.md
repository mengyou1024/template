# {{project-name}}

{{project_description}}

## Usage

This is a custom GitHub Action written in Rust and packaged in Docker.

### Inputs

- `message` - A message to echo (default: "Hello World")
- `name` - Name to greet (default: "User")

### Outputs

- `greeting` - The greeting message

### Example

```yaml
jobs:
  example-job:
    runs-on: ubuntu-latest
    steps:
      - uses: {{author_name}}/{{project-name}}@v1
        with:
          message: "Hello from GitHub Actions"
          name: "{{author_name}}"
```
