// -- Cargo Commands -- //
-> cargo init
-> cargo build
-> cargo check
-> cargo run
-> cargo new

// -- Crates Publishing -- //

// Release Profiles
Cargo has a dev and release Profiles
-> running cargo build uses the dev profile by default
-> running cargo build --release is for production
-> the dev profile is not optimized for runtime, but for compile time and it contains debug information
-> the release profile is the opposite
-> dev profile has an opt-level = 0 per default, e.g. see the Cargo.toml file
-> one can set those parameters specifcally
-> there are other parameters which can be set for dev and release settings

-> code in the /// section that is in a code markdown code block, i.e. ``` here ```, will be run when executing cargo test as a documenation test
-> this forces the documenation to be in sync with the code

// Documentation Comments
-> Documentation comments start with 3 slashes, i.e. ///
-> run cargo doc --open -> creates a html version of the documenation for the crate

-> comments that start with //! are documenting the element in whcih they are placed, i.e. if they are placed in lib.rs, then they are documenting lib.rs

// Commonly Used Sections
-> Error Section
-> Panics
-> Safety

// Documentation Comments as Tests

// Exporting a Public API
-> Making path available directly in the top level of the module when reading the crate
-> this makes importing easier for users who are importing the crate to use specific items from within the crate

// Crate Metadata
-> located in the Cargo.toml file
-> when publishing to Crates.io, we need to make sure that the name of the crate is unique

// Publishing to Crates.io
-> Crates.io works with github, that is why the changes always need to be commited with github
-> cargo publish will execute the publishing to crates.io
-> the code will always remain on crates.io, i.e. it will never be deleted
-> you can only alter it by changing the version and then cargo publish again to push updates and advances to the crate

// Removing Version from Crates.io
cargo yank --vers 0.1.0 -> this will disable the download of this version for new users; existing users will still be able to download it
cargo yank --vers 0.1.0 --undo will revert this command


// -- Cargo Workspaces -- //
-> help you in managing multiple related projects that are developed in tandem
-> packages in a workspace share a common dependency resolution by having a Cargo.lock file and various settings such as a profiles
-> they will also share the same target folder
-> this way we do not have to compile all dependencies separately in compiling the crate

// Creating a Workspace
-> we have to directly specify the directories we want to access in the packages' Cargo.toml file, i.e.
[dependencies]
add-one = {path = "../add-one"} in the adder Cargo.toml to access the contents of add-one package

// Tests in Workspace
-> cargo test in root of workspace will execute all tests defined in the packages
-> running test -p package_name will execute only tests in the package_name package

// Publishing Packages from Workspaces
-> you have to publish them distinctly, i.e. so they will be different crates within crates.io


// Installing Binary from cargo Install
-> use tools that are build from others developers and are available within crates.io, which one can execute
-> all binaries installed through cargo install will be installed in the folder installation root bin directory -> $HOME/.cargo/bin
-> make sure that this path is in the environment path so that one can access the binaries that have been installed
-> eg use cargo install ripgrep
-> then access it through rg --help

// Extending Cargo with Custom Commands
-> cargo something
