# Producing a minimal [Operating System](https://en.wikipedia.org/wiki/Operating_system) using [Rust](https://en.wikipedia.org/wiki/Rust_(programming_language))

## References

Throughout the semester we have been following this guide to write this minimal _OS_ in _RUST_ [Guide](https://os.phil-opp.com/).

## Overview

We have come up with the idea of writing a minimalistic _operating system_ using _Rust_. We don't have any target machine which the _OS_ targets specifially.

## Project Structure

We have divided the project into the following steps

### Pre-requisites

#### Disabling _Rust_ Standard Libraries

The first step we took was the creation of a new cargo project using the following command

```bash
cargo new rustOS --bin 
```

Here `--bin` indicates that the file being created is a binary file.

Once the creation of the project is complete, we are good to go with the next step that is of disabling the [_Rust_ standard libraries](https://doc.rust-lang.org/std/), because of the fact they use host _OS_ resources, and we don't need them for our _OS_.

```rust
// main.rs

#![no_std]

fn main() {
    println!("Hello, world!");
}
```

Once statndard libararies are disabled we cannot build use the same way which we normally do, if we try it will lead to an error just like the following.

```bash
error: cannot find macro `println!` in this scope
 --> src/main.rs:4:5
  |
4 |     println!("Hello, world!");
  |     ^^^^^^^
```

#### Adding Panic Implementation

Without panic implmenentation we won't be able to build this project therefore, we worte our own panic implementation. If we will build our project without writing panic implementation we would get the following error

```bash
> cargo build
error: `#[panic_handler]` function required, but not found
error: language item required, but not found: `eh_personality`
```

Here `eh_personality` can be handled by adding the following lines of code in _cargo.toml_ file and it is used to disable stack unwinding.

```rust
[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"
```

And for `#[panic_handler]` we need to write our own, here is how we managed it.

```rust
use core::panic::PanicInfo;
use os::println;

/// This function is called on panic outside of testing.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    os::hlt_loop();
}
```

#### Overwriting The Entry Point

In _Rust_ if we need to tell the compiler that we don't need to use the normal main function we used the following attribute.

```rust
#![no_main]
```

This will remove the `fn main() {}` from _Rust_, becasue at this point it doesn't make any sense of using it as we already have told _Rust_ that we won't be using _host OS_ resources. We will use our custom implmentaion of the _entry point_ instead, which is the following.

```rust
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
```

#### Adding a bare metal target

As stated that we won't be using any rourcse from the _host OS_ so we need a target upon which we can establish our _minimal OS_. The following command is used to achieve this goal.

```bash
rustup target add thumbv7em-none-eabihf
```

Once the target is installed we would need to build our sourcecode on the top of it, for that we would be using the following command.

```bash
cargo build --target thumbv7em-none-eabihf
```

### Setting up a Minimal Kernel

#### Installing nightly

To get our project complete we needed this version of _Rust_ beacuse it comes with libraires which are not present in the stable version of _Rust_, and we needed to those libraries to complete our project. By executing the following command we'll be able to install nightly.

```bash
rustup override set nightly
```

The successful execution of this command will overwrite the default installed verion of _Rust_ which we installed using `cargo new rustOS`.

#### Parameter

We rather are using _[QEMU](https://en.wikipedia.org/wiki/QEMU)_, a free open source emulator, to show the workaround of our _OS_. which takes an argument which we can term as a _bootloader_ for our OS, the arugemt which it accepts is a binary file. That comes from the execution of the following code snippet when running [`cargo build <option>`](https://doc.rust-lang.org/cargo/commands/cargo-build.html) command.

```bash
cargo build --target x86_64_os.json
```

By adding the following code snippet in our _cargo.toml_ file we wouldn't be required to explicitly write the optional feature after the `cargo build`  command. By adding this line we can build our project normaly and it will fetch the arguments from _cargo.toml_ file.

```rust
# in .cargo/config.toml

[build]
target = "x86_64-blog_os.json"
```

##### x86_64_os.json

```json
 {
    "llvm-target": "x86_64-unknown-none",
    "data-layout": "e-m:e-i64:64-f80:128-n8:16:32:64-S128",
    "arch": "x86_64",
    "target-endian": "little",
    "target-pointer-width": "64",
    "target-c-int-width": "32",
    "os": "none",
    "executables": true,
    "linker-flavor": "ld.lld",
    "linker": "rust-lld",
    "panic-strategy": "abort",
    "disable-redzone": true,
    "features": "-mmx,-sse,+soft-float"
}
```

The execution of the above code block will lead to an error, as shown in the following code snippet.

```bash
> cargo build --target x86_64-blog_os.json

error[E0463]: can't find crate for `core`
```

The error specifies that it was unable to find the `core` which we have already diabled in the start of our project, when we disabled the _Rust standard libraries_. Now to successfully compile the code we would need to add the following piece of code in out _cargo.toml_ file.

```bash
# in .cargo/config.toml

[unstable]
build-std-features = ["compiler-builtins-mem"]
build-std = ["core", "compiler_builtins"]
```

It will allow ot us use the `core` ragardless of the standard libraries being disabled, and it is a unsatable feature which comes with nightly.

### Creating a boot image

To use boot image we would need to install the bootimage package, which can be accomplished by using the following command.

```bash
cargo install bootimage
```

Once bootimage is installed successfully, we need to run it and for that we need a bootloader, and for that we need to install a component which comes from the execution of the following command.

```bash
rustup component add llvm-tools-preview
```

After the successfull installation of the `llvm-tools-preview` we can execute the boot image, for which we will use the following command

```bash
cargo bootimage
```

Once the binary file gets created we can pass to our _bare metal target_ using the followin command.

```bash
qemu-system-x86_64 -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-blog_os.bin
```

However, the execution of the following command will not show anything on the screen, becuse we aren't printing anything up till now.

#### Printing to the Screen

Here we use _VGA_ from our hardware in order to show the output of our code on the screen, the _VGA_ takes a byte string which we have passed our own custom byte string, and we are iterating over the byte string and passing it to the _VGA Buffer_ so that it can be displayed on the screen. Folliwng is the code snippet where we manage all these things.

```rust
static HELLO: &[u8] = b"Minimal OS!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
```

### Contributors

- [Maaz Ur Rehman Shah](https://github.com/Maazii){:target="_blank"}
- [Ishtiaq Naqi](https://github.com/ihnaqi){:target="_blank"}
- [Kundan Kumar](https://github.com/Kundan-404){:target="_blank"}
