# Website

This website is built using [Docusaurus](https://docusaurus.io/), a modern static website generator.

## Local Development

From the root of the repository:

```bash
nix run .#webStart
```

This command starts a local development server and opens up a browser window. Most changes are reflected live without having to restart the server.

## Build

From the root of the repository:

```bash
nix run .#webBuild
```

This command generates static content into the `build` directory and can be served using any static contents hosting service.
