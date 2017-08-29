# Basic Rust web template

**Work in progress**

**Note:** *Requires latest Rust nightly so that Rocket works properly.*

This code is meant to be used as a template for creating web applications in Rust. It includes  the
following:

 - An Outh 2.0 server implementation
 - A web cache in Redis
 - Gzip compression for responses
 - Automatic SASS minification to CSS at compile time (won't compile if SASS is not correct)
 - Automatic JavaScript minification at compile time (won't compile if JavaScript code is not corect)
 - CSS and JavaScript source maps if requested with the `source_maps` feature. This will enable easy
   SASS and JavaScript debugging from your favorite browser, since you will be able to see where in
   the source code the error happened even if what the browser executes is minified.

It also creates a security-first approach, following Rust's lead. Security is applied from the data
layer (database) to the business logic, to the frontend. Since this is still a work in progress,
many of these features are still missing.

## Documentation

Extensive documentation can be found through the code, but it's still work in progress. I expect to
improve it adding a complete guide on how to configure the website from the beginning.

## License

This code is distributed under the terms of both the MIT license and the Apache License (Version
2.0), at your option. See LICENSE-APACHE, and LICENSE-MIT files for details.
