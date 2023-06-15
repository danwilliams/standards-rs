# Coding standards for Rust

This document describes the coding standards chosen by [Dotfive](https://dotfive.co.uk)
for [Rust](https://www.rust-lang.org/) code, which are **highly opinionated**.
We choose to follow these standards instead of formatting with `rustfmt`, mainly
for reasons of readability.

It is intended to be a living document, and will be updated as
necessary. It is not intended to be a comprehensive guide to Rust, but rather a
set of standards that should be followed when writing code for Dotfive projects.
Notably, when contributing to other codebases, such as client or community
projects, we follow the rules they have defined rather than ones in this
document.

For the most part, these standards are based on common Rust community standards,
but there are some key differences. Additionally, this document clarifies our
approach in areas that are non-standardised and subjective.

This document is accompanied by example Rust files, and so this whole repository
is a working example of the standards described here, in the form of a
compilable, testable Rust project.


## Variation and evolution

The standards described in this document are not set in stone. They are
intended to be a starting point, and will evolve over time. There will be times
when deviating from the standards is the right thing to do, and that is fine,
providing there is a justifiable reason for doing so. The standards should be
followed unless there is a good reason not to.

If you are working on one of our projects and feel that the standards should be
changed, then please raise a question
describing the change you would like to see. If you feel strongly about it, then
please also provide a pull request for this repository. We will review the
changes, and if we agree with them then we will merge them in.

However, until the point that any suggested changes are approved and merged, the
current standards described in this document should be followed. You may
personally dislike some of them, and that is absolutely fine - it is impossible
to please everyone. It is important that we have a consistent approach to
writing code, and so we must all follow the same standards. You are free to use
your own preferred standards in your own projects, but **when contributing to
Dotfive projects, you must follow the standards described here**.

If any suggestions are rejected then we will explain why, and we will be happy
to discuss the reasons for the decision. We will not reject suggestions without
good reason, but ultimately it is a very opinionated process, and someone has to
make the final decision. If we do reject a suggestion, we expect that to be
accepted with good grace and not subject to argument or further debate, unless
there is some further information that has not been considered.


## IDEs and editors

Which IDE or editor is used will be fairly important in order to avoid
unnecessary friction in following certain aspects of the standards. It does not
matter exactly which tool is chosen, but it should at the very least be able to
be configured to support the indentation and whitespace requirements. Preferably
it will support Rust syntax and be able to validate Rust code, and ideally it
will allow for the configuration of our custom comment headers.

Our recommended IDE is [CLion](https://www.jetbrains.com/clion/), which is
available from [JetBrains](https://www.jetbrains.com/). It is a commercial
product, but [IntelliJ IDEA Community Edition](https://www.jetbrains.com/idea/)
is free and supports Rust, so that is a viable alternative. (Rust is supported
as a plugin to all JetBrains IDEs at present.) We recommend CLion over IntelliJ
purely because of focus - CLion is more targeted towards C/C++ and Rust, whereas
IntelliJ has a wider reach.

We do not currently recommend [Visual Studio Code](https://code.visualstudio.com/),
simply because its interaction with Rust is not as smooth as it could be, and it
misses out on many of the advantages provided by JetBrains IDEs. However, it is
a perfectly viable alternative, and is free.

A worthy mention is [Sublime Text](https://www.sublimetext.com/), which is a
commercial product, but is free to evaluate. It is a very powerful editor, and
is highly configurable, but it does not have the same level of support for Rust
as the JetBrains IDEs.

Some of the standards described in this document have settings provided for
JetBrains IDEs. If you choose to use a different tool then you will need to
configure it to match the settings described here, or otherwise perform the
necessary actions manually.

*Note: There are some screenshots showing our standards in action in the
[`screenshots`](screenshots/) directory. See the [Highlighting](#highlighting)
section for more details.*

### Fonts

Our custom comment headers use symbols from [Font Awesome](https://fontawesome.com/),
which is available in the [Nerd Font](https://www.nerdfonts.com/) range. We
therefore suggest that you pick one of the Nerd Fonts for your editor, so that
you are able to see the symbols correctly.

As an aside, it's worth also configuring your terminal with a Nerd Font too, so
that you can see the symbols correctly in any command-line interactions.

### Highlighting

If you are using a JetBrains IDE, then you can import the provided settings from
the `IDEs/JetBrains` directory in this repository. This will configure the IDE
to use the correct highlighting for the custom comment headers.

Note that the settings rely upon you having the [Better Highlights plugin](https://plugins.jetbrains.com/plugin/12895-better-highlights)
installed. This is a free plugin, and is available from the JetBrains plugin
repository. It is not necessary to use this plugin, but if you do not then you
will not see the custom comment headers highlighted correctly.

There are some screenshots showing our standards in action with this
highlighting applied in the [`screenshots`](screenshots/) directory:

  - [Main code routine](screenshots/main.png)
  - [Enums](screenshots/foo-enums.png)
  - [Structs](screenshots/foo-structs.png)
  - [Methods](screenshots/foo-methods.png)
  - [All styles](screenshots/styles.png)

## Code formatting

This section refers to the formatting of Rust code, rather than Markdown files.
The formatting of Markdown files is covered in a later section.

### Indentation

We use **tabs for indentation**, with a tab width of 4 spaces. Indentation is
considered to be any whitespace that occurs at the start of a line. Once a
non-whitespace character has occurred, indentation is considered to have ended
(but see the notes on [comments](#comments) for some specific rules relating to
them).

Of particular note here is that example code in inline Rustdoc documentation
should be indented by 4 spaces, and should not use tabs.

### Alignment

We use **spaces for alignment**. Alignment is considered to be the vertical
positioning of elements of code, such as assignment operators, that occur
part-way through a line of code.

We use alignment to improve readability, and to make it easier to spot
differences between lines of code. We align assignment operators, and the fields
in struct definitions. If there are similar elements on consecutive lines of
code, then we align them to make it easier to read.

Please line up assignment operators in nearby lines of code, unless there is a
good reason not to.

*Note: Remember that the intent is to **improve** readability, and not impede
it. If aligning certain elements makes the code harder to read, then that might
imply that the alignments should be constrained to a more localised area.*

The one major downside of alignment is that it can cause lines to change due to
whitespace adjustments when they have no functional differences. This can make
it harder to see what has actually changed in a commit. However, we feel that
the benefits of alignment outweigh this downside, the reason being that
maintaining readable code is more important than avoiding the slight pollution
of commits.

##### Examples

This is an example of how to align assignment operators:

```rust
let data    = vec![1, 2, 3];
let average = 0.0;
```

This is an example of how to align the `=>` in match arms:

```rust
match result {
    Ok(value)  => {},
    Err(error) => {},
}
```

This is an example of how to align the fields in struct initialisation:

```rust
Point {
    x:        0,
    y:        0,
    settings: Settings::default(),
};
```

This is an example of how to align similar terms in consecutive lines of code:

```rust
let a = foo + bar * baz / 100.0;
let b = a   + bar * baz /  10.0;
```

### Line breaks

We use **Unix-style line breaks** (LF, `\n`) rather than Windows-style line
breaks (CRLF, `\r\n`). This is the default for Rust, so you should not need to
do anything special to achieve this.

### Line length

We recommend column guides at 80 and 120 characters. For the most part, you
should aim to keep lines of code to 80 characters or less, but 120 is considered
to be the maximum. In very rare cases it might be that readability is improved
by going beyond 120 characters, but this should be avoided if possible.

### Whitespace

You should configure your editor to NOT remove trailing whitespace. You should
not have any at the end of lines of code, but comment headers rely on trailing
whitespace, and we prefer to maintain indentation on blank lines.

### Trailing commas

As is the standard convention in Rust, we use trailing commas in lists of items.
This means that you should always have a comma after the last item in the list.
This improves compatibility with version control systems, as a new line can be
added without modifying the previous one by adding a comma.

The exception to this rule is where a new item is impossible, which is deemed to
be the case when using the `..` operator, for instance to provide defaults. In
this case, the trailing comma is omitted, as this will always be the last item.

### Comments

Broadly speaking, there are structural comments, non-structural comments, and
Rustdoc documentation comments. Structural comments are used to create sections,
and non-structural comments are used for programmer notes. Rustdoc documentation
comments are used to create browsable documentation for the code.

This section deals with the strucural and non-structural comments (using `//`),
rather than Rustdoc documentation comments (using `///` and `//!`). For Rustdoc
documentation comments, see the [Code documentation](#code-documentation)
section.

Non-structural comments in code fall into two categories: whole-line comments,
and inline comments. Whole-line comments are comments that occur on a line by
themselves, and inline comments are comments that occur on the same line as
code.

#### Structural comments

Structural comments are used for creating sections, and have special formatting
symbols to allow colouring to be applied to make them stand out. They all start
with a comment symbol (`//`), followed by a special symbol, followed by *two*
tabs, followed by the comment text. The special symbols are:

  - `` - Divider
  - `` - Section																	
  - `` - Sub-section																
  - `` - Function																
  - `§` - Trait																	
  - `` - Struct																	
  - `` - Macro																	
  - `` - Enum																	
  - `` - Error
  - `` - Test																
  - `` - Method																
  - `` - Properties															
  - `` - Logic section									
  - `` - Logic/properties sub-section					
  - `` - Line of interest								
  - `` - Query		
  - `` - Unsafe		

*Note that you may need to view these in the Markdown source code, using an
appropriate Nerd Font, to see the symbols correctly. A [screenshot of the
symbols](screenshots/symbols.png) is also available.*

These each belong to a category, and the category determines the trailing
whitespace requirements.

  - Divider: No trailing whitespace.
  - Query, unsafe: Tabs to make the block 30 characters wide.
  - Everything else: Tabs to align to the 80th column.

Some of the structural comments are mandatory, such as those included before
core constructs. The logic sections are optional, and are useful for breaking up
longer sections of code to make them easier to read.

##### Examples

You can see these demonstrated in the `examples/styles.rs` file, and there is a
[screenshot showing all the styles in action](screenshots/styles.png) in the
[`screenshots`](screenshots/) directory.

*Note: For more screenshots see the [Highlighting](#highlighting) section.*

#### Whole-line comments

Whole-line comments should be composed of the comment symbol (`//`) followed by
a tab, followed by the comment text. They will quite often span multiple lines,
in which case each line repeats the same prefix. The rationale is that the
comment symbol is "invisible" as it denotes the nature of the line, and so the
indentation rules are still in effect for the comment text.

Whole-line comments are dealt with as a block for line-length purposes. The rule
is that if the comment symbol is located at the zeroth or first tab stop, then
the comment text should be wrapped at the 80th column. If the comment symbol is
located at the second tab stop or beyond, then the comment text should be
wrapped at whichever column would be the 80th column if the comment symbol was
at the first tab stop. The best way to do this is to write your comment text
when the comment symbol is at the first tab stop, and then indent the comment
block to the correct tab stop afterwards. The rationale behind this is that code
is often refactored, and things move around. It is annoying to have to reflow
comments every time this happens, plus for readability a set width is nicest for
the comment blocks. The only other consideration is that the comment text should
go beyond the 120th column - if it will do so by following the above rules, then
this is likely an indication that your code is nested too deeply, and you should
consider refactoring it.

Comments should be used sparingly, and only when necessary, to help explain code
that is otherwise difficult to understand, or, more likely, the reason for a
particular approach being taken. (Code that is hard to understand is often, but
not always, a sign of poorly-written code, so refactoring should be considered
ahead of merely commenting.) Comments should typically not be used to explain
what the code is doing, but rather why it is doing it. They should be used to
explain the rationale behind a particular approach, or to explain the purpose of
a particular function or module.

##### Examples

This is an example of a whole-line comment:

```rust
//  This is a whole-line comment. It is composed of the comment symbol followed
//  by a tab, followed by the comment text. It spans multiple lines, but each
//  line repeats the same prefix.
```

#### Inline comments

Inline comments should be separated from the code by two or more spaces,
followed by the comment symbol (`//`), followed by two spaces, followed by the
comment text. If there are other inline comments in the same region of code,
they should be vertically aligned. Inline comments are not allowed to span
multiple lines.

##### Examples

This is an example of an inline comment:

```rust
let foo = bar * 10;  //  This is an inline comment.
```

## Filesystem layout

A typical Rust project should have the following structure:

  - `src/`
    - `main.rs` or `lib.rs`
    - *other modules*
    - `tests`
      - *module tests*
  - `tests/`
    - *integration tests*
  - `Cargo.toml`
  - `README.md`
  - `LICENSE`

A multi-crate workspace should place each crate under a `crates/` directory,
with the workspace `Cargo.toml` file at the root of the repository. The
crate should each follow normal structure in their own subdirectories.

If a module grows too large, it should be split into multiple files. These
should be placed in a subdirectory named after the module, and the module should
publish whatever internals are needed. However, avoid deeply-nested submodules,
as they complicate the crate structure. Typically a module should not have more
than two levels of modules (i.e. one nested level of submodules).

We take the approach of placing unit tests in separate files to the code they
test, and making the tests submodules. For more information on this, see the
[Testing](#testing) section.


## Code documentation

Documentation should be written using Rustdoc documentation comments. These
comments are written using the documentation comment symbols (`///` and `//!`),
and should be placed immediately before the item they document (except for
`//!`, which documents the "thing" they are in). They should be written in
British English, in present imperative tense, and in the third person, avoiding
any personal pronouns.

You should assume that the reader of your documentation already knows Rust and
will understand your code, and therefore the focus should be to explain what was
built, how to use it, and anything unusual or not obvious about it.

You should always document all structs, struct fields, enums, enum variants,
traits, functions, methods, and macros. You should typically also document
modules, although this can be skipped if the module provides one primary struct
and the purpose is clear through its documentation. Tests do not require
documentation.

If you link to other items, you should use the `[]` syntax, and leave as much as
possible up to Rustdoc to generate the links. In other words, do not use HTML
links, do not manually specify anchors, and do not use the `[name](link)` syntax
at all unless you need to override the link. This is because Rustdoc will
automatically generate the correct links for you, and do so in a way that is not
otherwise achievable.

Wrap your documentation at the 80th column, regardless of whether it started as
indented or not (you should find that your documentation blocks are never
indented by more than one tab). The exception here is links: the title of the
link is required to be fully constrained inside the 80 columns, but the link URL
can extend past this. If the link URL would go beyond 120 columns, wrap before
the link title and place the entire link on its own line.

### Structs

Structs should be documented comprehensively for the most part, varying
depending on their importance.

Struct fields should have at least a minimal description.

### Enums

Most enums are fairly self-explanatory, and so only tend to require a minimal
description.

Enum variants should have at least a minimal description.

### Traits

Traits should be documented comprehensively, as they are the primary interface
for a type.

You should document trait methods on the trait, and not on the implementing
type. This is because the trait is the primary interface, and so the
documentation should be there.

### Functions and methods

Functions and methods should have a number of key sections that help inform
about their behaviour, which are not applicable to other items. These should be
presented in sections using Markdown headings. Not all of the sections are
always applicable, but think about which may be needed and include them if
they are relevant.

  - Parameters
  - Examples
  - Panics
  - Safety
  - See Also

Notably, Markdown lists should follow two individual styles, depending on the
context. For parameters and see-also links, the list should use `*` style
bullets. However, for lists that are part of general documentation prose, they
should use `  - ` style bullets, i.e. two spaces for indentation, followed by a
dash, followed by a space.

The "See Also" section should always be a list of links to other items, and
not contain any prose.

*Note: You should document trait methods on the trait, and not on the
implementing type.*

### Macros

Macros should be documented in the same way as [functions and methods](#functions-and-methods),
as they are basically a special type of function.

### Modules

Modules do not require documentation if they provide one primary struct and the
purpose is clear through its documentation. Otherwise, they should be documented
comprehensively.

### Tests

Tests do not require Rustdoc documentation.


## General documentation

All documentation that is not related specifically to the code, and cannot live
inside the source code files as Rustdoc documentation, should exist in Markdown
files. The primary example of this is the main `README.md` file that all
projects should have in their root. If it is necessary to split the
documentation into multiple files, then these should be linked to from the main
`README.md` file, unless there is very good reason not to. This helps maintain
discoverability.

All Markdown should follow general Markdown formatting rules, in similar fashion
to the [Rustdoc documentation](#code-documentation). Broadly speaking, use
correct headings and other constructs, and align to 80 characters except for
links (links are allowed to extend out further).


## Testing

There are essentially three kinds of test that we are interested in: unit tests,
integration tests, and compilation tests. Compilation tests are a special case,
as they can only be achieved as a form of integration test, and so are placed
under that area of the filesystem.

### Unit tests

Unit tests should be placed in a `tests/` directory alongside/under the module
they are testing. So for instance, if your module is `src/foo.rs`, then your
unit tests should be in `src/tests/foo.rs`. The unit tests should be set up as a
submodule of the module they are testing, which allows testing of private
aspects of the module. This means that every module has a submodule called
`tests`.

Typically your tests should just sit directly inside the test module, but in
some cases it may be necessary or useful to create submodules in the tests
module.

Each unit test should test one conceptual "thing". That does not mean one unit
test per method, and not does it mean one assertion per method. Rather, it means
that you should consider the outcome of your test, and try to make it atomic. If
you have an assertion checking a valid state and another one checking an invalid
state, it may be best to create two test methods, each with just one assertion.
Conversely, if you have a whole load of assertions checking different valid
variations, or maybe different fields of a struct under one use case, then these
most likely belong together in the same test method.

Name your test methods after the thing they are testing, e.g. if you are testing
a method called `foo()` then call your test `foo()`. Any variations that need to
be tested should then add a suffix of two underscores, followed by a short
description of the concept - for instance, `foo__valid()` and `foo__invalid()`.

Try to avoid code duplication by placing common code, such as that used to set
up and prepare the test environment, into separate (non-test) functions. If your
common methods belong in the `foo::tests` module, then prefix their names with
two underscores, e.g. `__setup()`. However, if they have wider purpose then
place them into a top-level module called `tests`, in which they can exist
without the double-underscore prefix requirement.

### Integration tests

In Rust, integration tests are placed in a `tests` directory as a sibling to the
`src` directory. The tests run from here will not be able to test any of the
internals of the application, and should test it from the outside in.

The approach to integration tests should broadly follow the same rules as for
[unit tests](#unit-tests).

### Compilation tests

In some cases there may be things we need to test that, if they fail, would
result in compilation failures. In context, this is not the same as something
generating a panic at runtime - it is a panic at compile-time. Therefore, such
cases cannot be caught in the usual way by using `#[should_panic]`. In order to
test for expected compilation errors, we use [`trybuild`](https://crates.io/crates/trybuild).

The way that `trybuild` works is to wrap the compilation process and catch the
results. It then compares the results against ones already stored. If they
match, the test is successful. This means that testing for successful or
unsuccessful builds use exactly the same approach. It also means that a build
test is a special kind of integration test, and gets run as an integration test
from the integration tests folder. However, it gets *triggered* from a unit
test (usually - although it could also be triggered by an integration test).

We tend to add a `tests/compile_fail` subfolder, and place the files to run
there (along with the expected output), and then execute them from one function
in the unit test suite. Multiple compile tests can be run at once that way, if
needed. The normal rules should be followed when deciding what should constitute
a separate test and give rise to an individual integration test function.


## Version control

A full description of Git etiquette is currently outside the scope of this
document, but in basic summary you should create meaningful, semantic,
carefully-stage commits, with appropriate commit messages and descriptions,
following the Dotfive Git standards.

Specifically to the context of Rust projects, you should try not to push any
commits that individually fail tests. As far as possible, each commit should be
able to pass testing and linting checks. Certainly by the time your branch is
submitted for PR, all tests should be passing along with all linting checks.


## Development process

The general development process should go something like this:

  1. Write your Rust code, using idiomatic approaches, and obeying the Dotfive
     coding standards.
  2. Write documentation for all new code you produce, in line with the
     requirements in this document.
  3. Ensure that you have complete test coverage for your additions or changes.
     If you are writing new code, then ideally you will use TDD to the greatest
     extent possible. If fixing a bug, consider if you need to add new tests to
     check for the specific thing you have fixed.
  4. Run `cargo build`, `cargo test`, `cargo clippy`, and `cargo doc` to check
     that the application builds correctly, the tests pass, and that there are
     no linting or documentation errors to correct.
  5. Commit and push.

We do not expect there to be zero warnings, although it should be strived for.
When a codebase is in the process of development, there can often be parts that
are as-yet unused, and in that case it is fine to ignore the related warnings
about those areas. The following flag can be useful to disable those warnings in
such cases:

```sh
RUSTFLAGS="-A unused" cargo clippy
```

Similarly, there may be areas of code that generate warnings but for which there
is good reason to not follow the suggested changes. In such cases, if the
approach is considered permanent, add an annotation to disable the associated
warning for that specific instance.

Otherwise, try to clean things up to eliminate or at least minimise warnings.
And, it should go without saying - **your code should always compile**.


## Templates and examples

A number of example files are provided for reference, and to use as templates.
They demonstrate the coding styles and standards described in this document.

### Root directory

  - `Cargo.toml` - This provides a typical Rust project configuration file. The
    appropriate license should be chosen and placed into the repository. For
    open-source licenses, the `license` field can be used; for proprietary
    licenses, the `license-file` field needs to be used, and the required
    license file placed in the root of the repository as `LICENSE`.
  - `LICENSE-MIT` - The MIT license. This is a permissive open-source license.
  - `LICENSE-proprietary` - A proprietary license. This is a placeholder file
    that should be replaced with the actual license file if it differs.
  - `src` - The source code directory. This is where all the Rust code lives.
  - `tests` - The integration tests directory. This is not present in this
    repository, and should be created if needed.

### `src`

  - `main.rs` - A typical Rust application entry point. This is just a simple
    "hello, world" program, but shows a standard layout.
  - `foo.rs` - A typical Rust module file. This demonstrates a number of aspects
    of the coding style, and includes a main struct, a settings struct, an enum,
    an error, and Rustdoc documentation stubs.
  - `tests/foo.rs` - A typical unit test file. This demonstrates a standard
    layout in line with the required coding style, and includes some tests for
    the `Foo::new()` method.
  - `tests.rs` - A test helper module, which contains an example of a common
    test helper function. Note that because of the `#[cfg(test)]` declaration
    in the `main.rs` file, this module is only compiled when running tests, and
    is not otherwise available.
  - `styles.rs` - A list of all the comment header styles that are available,
    for quick reference.


