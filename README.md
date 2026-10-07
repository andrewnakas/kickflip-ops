# Kickflip Ops

A military shooter where you can drop onto a skateboard any time, in a city with a block quarry you can dig and build.

**Play in your browser:** https://andrewnakas.github.io/kickflip-ops/ · [aimashups.com/play/kickflip-ops/](https://aimashups.com/play/kickflip-ops/)

Kickflip Ops is a military shooter you can skate. Drop onto a board mid-firefight, kickflip over cover and land a KICKFLIP KILL, then dig into the block quarry for cover or blow it apart with TNT and airstrikes. Bots patrol, flank and shoot back; killstreaks earn a UAV, a care package and an airstrike.

## A clean-room tribute
Kickflip Ops is a tribute to [the 2010 Rust Rewrite Mashup (MW2, Skate 3 and Minecraft) by chasmlol](https://github.com/chasmlol/2010-rust-rewrite-mashup), one of the most-starred AI game mashups. That project runs on the original games and needs your own copies. This one recreates the mechanics from observed behaviour and published values, with original code and CC0 assets, so it runs in a browser with nothing to install and no game files. It isn't affiliated with the original project or with any publisher. See `docs/specs/` for the behaviour specs.

## Run
```sh
cargo run                      # native
cargo test                     # sims and rules
trunk serve --release          # web build at http://127.0.0.1:8080
```
CI (`.github/workflows/ci.yml`) tests, builds the WebGL2 and WebGPU web builds, publishes them to GitHub Pages, and builds Windows, macOS and Linux releases for `v*` tags. A headless-browser smoke test (`smoke.yml`) loads the published build after every deploy.

Built in Rust on [Bevy](https://bevyengine.org) and Rapier, sharing its engine with [GameMash](https://github.com/andrewnakas/GameMash).

## Asset credits
| Asset | Author / source | License |
|---|---|---|
| Textures: concrete_floor_02, concrete_pavement, asphalt_02, red_brick_03, blue_metal_plate, plywood, dirt | Poly Haven (polyhaven.com) | CC0 |
| Sky HDRI: skate_park | Poly Haven | CC0 |
| Character + animations: Universal Animation Library (UAL1 Standard) | Quaternius | CC0 |
| Human bodies, hair, eyes: Universal Base Characters (outfits painted by tools/paint_outfits.py) | Quaternius | CC0 |
| AK rifle and pistol models | loafbrr (OpenGameArt) | CC0 |
| Gunshot recordings (sks, cz) | Vincent Sevedge (OpenGameArt "gunshot-sounds") | CC-BY 3.0 |
| Reload sounds, explosion, engine loop | OpenGameArt contributors | CC0 |
| Skateboard roll / land / ollie | FOSSarts, Freesound (pack 41196) | CC0 |
| Footsteps and impacts | Kenney (kenney.nl) Impact Sounds | CC0 |
