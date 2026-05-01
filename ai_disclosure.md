# AI Disclosure

In this project I have used AI coding tools, primarily OpenCode with Claude models to build out this library.

This document is to serve as a responsible disclosure for how I have used these tools so that users of the library are aware.

## Human driven aspects

I have not attempted to "vibe code" this library. It has taken a lot of hours of refinement and refactoring to get to the stage it was for the `0.1.0` release.

The interface for using this library has been designed by me, from my experience working with conversational projects and human entered date information.

I created the initial spec, and then had the AI tool iterate on that spec to expand the details of it.

I have made the decisions about how to structure details like the config and the features of the library.

I have attempted to review any and all generated code as much as possible.

## AI Tooling

The AI tooling has been most extensively used in the following ways:

- Creating documentation for the library
- Implementing somewhat boilerplate code
  - The Levenshtein distance implementation was primarily AI tool generated
  - The Tokenisation approach was AI generated
- Expanding on models and implementing builder pattern helpers based on my instructions
- Expanding on the test cases initially given
- Generating a lot of the examples, particularly the numbered ones
- Generating tests from the test cases provided
- Expanding on the word number handling for transforming strings like `eighteenth` to `18`

These are the main areas I have used these tools as a means for creating this library.

I have attempted to instruct the tools to follow the best practices I feel are good for working with Rust and these tools, and I have encapsulated those in the [AGENTS.md](./AGENTS.md) file. Take a look to see what I have instructed the model to do in terms of Rust best practices.