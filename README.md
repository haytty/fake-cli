# Fake-CLI

## About

The fake-cli project is a powerful utility for generating structured text format test data. It's especially useful for
developers who require randomized data for testing or data analysis. This project leverages the power
of [fake-rs](https://github.com/cksac/fake-rs), a popular randomized data generation library.

## Usage

To use the fake-cli utility, you will need to define a \`fake_definition_json\` like the example below:

```json
{
  "example_word": {
    "fake_type": "word",
    "lang": "JA_JP"
  },
  "example_digit": {
    "fake_type": "digit",
    "lang": "EN"
  },
  "example_sentence": {
    "fake_type": "sentence",
    "lang": "JA_JP",
    "min": 1,
    "max": 5
  }
}
```

You can then feed this definition to the fake-cli utility through the \`--json\` switch. The utility will generate
random JSON data based on your specifications. For example, given the \`fake_definition_json\` above, the utility might
output:

```json
{
  "example_digit": 4,
  "example_sentence": "qui et maiores.",
  "example_word": "et"
}
```

## fake_type Configuration

The available `fake_type`'s are:

- Word
- LastName
- Words
- Boolean
- Digit
- Sentence
- NumberWithFormat
- Array
- Map

Note: `Array` and `Map` are specific to fake-cli.

The `fake_type` can be any callable method from fake-rs, and it is ready to use when converted to snake_case.

## License

This project is licensed under the terms of the [MIT License](LICENSE) (or whichever license you have chosen).