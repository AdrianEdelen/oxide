# oxide


“An idiot admires complexity, a genius admires simplicity, a physicist tries to make it simple, for an idiot anything the more complicated it is the more he will admire it, if you make something so clusterfucked he can't understand it he's gonna think you're a god cause you made it so complicated nobody can understand it. That's how they write journals in Academics, they try to make it so complicated people think you're a genius”

― Terry Davis, Creator of Temple OS 

Oxide has a few primary goals in mind. 
    - simplicity, oxide is designed to be small and simple. the kernel is not meant to be this huge piece of code with support for anything that can be thrown at it. Instead, the kernel itself is designed to be able to be worked on by the user, tweaked and customized for the users hardware and use case. Don't need a filesystem, take it out (it might be running in userspace anyway)

    - oxide is also designed with the idea of a single user environment. Modern desktop operating systems are still built off the back of server and mainframe systems. this doesn't make sense to me. when looking at any smart phone or mobile OS, there is typically one user, no sign in page, no user perms to deal with. oxide, most people use computers like this now. oxide is built from the ground up to be a single user experience. that isn't to say it can't be used by more people. but why add the overhead.



Due to the nature of kernel development, there are some odd steps that must be taken in order to build the project

Additionally, for testing, it is Highly recommended that Qemu is used with this configuration.

Qemu is used for unit tests, integration tests, and for running the software. Otherwise, you will have to deploy the kernel to bare metal to run.

first build may take a long time since core needs to be compiled, this project does not use the std lib. (since we don't have it on bare metal)
Windows

Download Qemu and make sure that it is in your PATH make sure that the target is x86_64-os.json in config.toml, this will build for bare metal x86-64 when no target is specified. run the command rustup override set nightly in your terminal. this project requires expiremental features that may not have been implemented in the current version of rust.

when running the bootimage crate should start qemu automatically.

cargo test to run all tests (unit and integration). cargo build to built cargo run to start Qemu and load the kernel.
