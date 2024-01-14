# cdevents rust code generator

Goals: generate rust code for cdevents from jsonschema provided as part of cdevents specs.

- The generator take read jsonschema as json apply them to a set of templates
- The generator is very basic (no json schema semantic, no `$ref` resolution) like [eventuallyconsultant/codegenr: Fast handlebars templates based code generator, ingesting swagger/openapi and other json/yaml documents with $refs, or graphql schema, outputs whatever you template](https://github.com/eventuallyconsultant/codegenr/)
- The generator is currently used to generated Subjects

## Why not use a jsonschema to rust generator?

- I tried some (like ) and they failed (no error), maybe too early, not support for the version of jsonschema used by cdevents (often they support jsonschema draft-4)
- The json schema (v0.3) are not connected, so lot of duplication (context,...), so classical generator will create as many Context type as Event type,...

## Run

To generate the `subjects` into sibling crate `cdevents/src/generated` from content of `cdevents-spec/schemas`, from root workspace

```sh
cargo run -p generator -- --help
cargo run -p generator -- --templates-dir "generator/templates" --jsonschema-dir "cdevents-spec/schemas" --dest "cdevents-sdk/src/generated"
```