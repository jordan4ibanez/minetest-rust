# Minetest Rust
Minetest is a free open-source voxel game engine with easy modding and game creation.

Copyright (C) 2010-2023 Perttu Ahola celeron55@gmail.com and contributors (see source file comments and the version control log)

### **This is not an official build of minetest.**

-----

This is a rebuild of minetest from the ground up by jordan4ibanez.

If you would like to help me with this herculean undertaking in real time:
https://discord.gg/Z2wCscTB4F

-----

## Current package usage:
- clap - CLI parsing.
- configparser - Parsing .conf files.
- ctrlc - Catching termination events and elegantly exiting the program.
- message-io - UDP networking.
- spin_sleep - Main loop speed control.
- spin_sleep_util - Assistant to spin_sleep.
- sdl2 - Windowing library. (but could be used for more things)
- glam - An incredible math library.
- env_logger - An elegant logging library.
- log - Used in conjunction with env_logger.
- pollster - A micro library which allows blocking a thread while a future completes.

- wgpu - Graphics multiplexer. (Vulkan, OpenGL, WebGPU, metal)
- wgpu_sdl_linker - A micro library which allows safely linking wgpu with SDL2.
- bytemuck - Used in conjunction with wgpu to cash vertex data.
- 
- rand - For doing random things.

##### Packages to be implemented:
- rusqlite - SQLite3 database.
- sea-query - SQLite3 query builder.
- serde - Serialization and deserialization of data.
- serde_bytes - Same as serde.


##### Experimental packages for testing:
- quote - Common Lisp code as data features.
- syn - Common Lisp code as data features.

## Current progress

- Client and Server monolithic framework
- Basic UDP networking complete with timeout integration
- Client and Server LuaEngine which implements LuauJIT
- Elegant handling of termination signal to program
- Elegant handling of frame/tick timing
- Window opened with SDL2
- I'm probably forgetting something

## Next up
- Set up wgpu
- Render a triangle with wgpu 
- Rendering some text in the window
- A basic GUI (minetest formspec/HUD)
- Client settings that can be modified during runtime
- SQLite3 Server framework with rusqlite
- Writing hello world into the SQLite3 database
- Use sea-query to write hello world into the SQLite3 database
- More SQLite things???
- Serde serialization integration with minetest data structs
- I'm probably forgetting something again

-----

(keeping this here for help)

https://blessed.rs/crates

https://github.com/rust-unofficial/awesome-rust

https://arewegameyet.rs/

## mold
An ultra-fast linker which you can optionally use!

Please install mold then paste the following in `.cargo/config.toml` if you want to use it.
```toml
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=/usr/bin/mold"]
```

I do not use windows or mac, if you would like to test it with those, feel free to add instructions.

github repo: https://github.com/rui314/mold

mold available repo packages: https://repology.org/project/mold/versions

-----

## wgsl-analyzer

Currently the best wgsl analyzer I can find for vscode is wgsl-analyzer.

github repo: https://github.com/wgsl-analyzer/wgsl-analyzer

-----
