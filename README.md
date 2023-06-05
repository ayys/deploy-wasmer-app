# deploy-wasmer-app

This action deploys an app to wasmer.io using their GraphQL interface. I am
choosing to not use the `wasmer-deploy-cli` crate for this because it is in
flux right now <2023-06-05 Mon>.

I am open to contributions that will add option to use `wasmer-deploy-cli` or
`wasmer-cli` to deploy the app.


## Inputs

| name       | description                             | Type     | default value            |
|------------|-----------------------------------------|----------|--------------------------|
| `token`    | Token to use when publishing the app    | Required |                          |
| `registry` | Name of the registry                    | Optional | `wapm.io`                |
| `name`     | Name of the app `namespace/appname`     | Optional | __name field in config__ |
| `strict`   | Verify the package is installed         | Optional | `true`                   |
|            | make sure the package-version is        |          |                          |
|            | explicitly specified in config          |          |                          |
| `config`   | File to read the deployment-config from | Optional | `deploy.yaml`            |


```yaml

name: Deploy app to wasmer.io
on:
  push:
    branches:
      - main
jobs:
  publish:
    runs-on: ubuntu-latest
    needs: deploy
    steps:
      - uses: actions/checkout@v3
      - uses: dbanty/deploy-wasmer-app@v1
        with:
          registry: ${{ secrets.PRODUCTION_ENDPOINT }}
          token: ${{ secrets.AUTH_TOKEN }}
```


## TODO

- [ ] Update this README with the details of this action
- [ ] Update inputs/outputs in `action.yaml`
- [ ] Implement the action's logic in `src/main.rs`
- [ ] Rename the default Git branch to `v1` (instead of `main` or `master`. This
  helps with potential future breaking changes. **PROVIDED ACTIONS WILL NOT WORK
  UNTIL YOU DO THIS**
