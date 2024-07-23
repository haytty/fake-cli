/// This `lang` module provides enums and functionality for defining and selecting the languages to use within the fake data.
pub mod lang;

/// This `fake_type` module provides the `FakeType` trait and its associated implementations. This functionality is used to generate individual pieces of fake data.
pub mod fake_type;

/// The `fake_definition` module provides the `FakeDefinition` type and associated functionality. A `FakeDefinition` represents an entire set of fake data, derived from one or more `FakeType`s.
pub mod fake_definition;

/// This private `fake_definition_element` module provides the functionality for parsing and handling elements of a `FakeDefinition`.
mod fake_definition_element;