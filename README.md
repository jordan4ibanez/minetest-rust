# Minetest Rust
Minetest is a free open-source voxel game engine with easy modding and game creation.

Copyright (C) 2010-2023 Perttu Ahola celeron55@gmail.com and contributors (see source file comments and the version control log)

### This is not an official build of minetest.

-----

(keeping this here for help)
https://blessed.rs/crates


## mold
An ultra-fast linker which you can optionally use!

Please install mold then uncomment the following to `.cargo/config.toml`` if you want to use it.
```toml
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=/usr/bin/mold"]
```

I do not use windows or mac, if you would like to test it with those, feel free to add instructions.

github repo: https://github.com/rui314/mold

mold available repo packages: https://repology.org/project/mold/versions

-----

A rebuild of minetest from the ground up by jordan4ibanez.

If you would like to help me with this herculean undertaking in real time:
https://discord.gg/Z2wCscTB4F

-----
