# rtweekend-rs

![render of a random scene of spheres with a flat color, metal reflections and glass reflections/refractions, final output of the RayTracingInOneWeekend book. Except without blur/deph of field](/weekend_final_noblur.png)

An implementation of [RayTracingInOneWeekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) in Rust that I made while reading the book

The canonical C++ implementation can be found [here](https://github.com/RayTracing/raytracing.github.io/tree/master/src/InOneWeekend)

To build this, you need [cargo](https://github.com/rust-lang/cargo), the rust package manager, and the recommended way to install it is via [rustup](https://rustup.rs/), the rust toolchain installer.
Note that this crate only optionally depends on the [rayon]() library for parallel iterators, and otherwise has no depencies (Other than the rust std lib, and either libc for rand() or BCryptGenRandom from the windows api)

## Features

The crate includes a few features (Compile time flags):

- parallel: Parallelize scanlines using rayon. Uses a thread local RNG state
- dyn_hit: Use trait objects (Dynamic dispatch/vtables) for hittables. This is what the book does, but since we only use spheres, disabling this replaces dyn Hittable's with Sphere's
- dyn_mat: USe trait objects (Dynamic dispatch/vtables) for materials. Without this feature, an enum (Essentially a tagged union) is used
- wincrypt_rand: Use the [BCryptGenRandom] windows API instead of libc rand(). This is what I initially used (I wanted to avoid pulling in a dependency for the RNG), before I realized I could just use libc rand.

By default only dyn_hit and dyn_mat are enabled, to be as close as possible to the canonical C++ implementation. In my tests, using the parallel and wincrypt_rand features (With no trait objects) was the fastest. You can run with those features executing the following:

```
    cargo run --release --no-default-features --features wincrwpt_rand,parallel > image.ppm
```

Please note that the feature specific code was added at the end and without much care for readability, so it's somewhat ugly

## PPM Viewer

The book works with [PPM](https://en.wikipedia.org/wiki/Netpbm#PPM_example) files, and that's what this program outputs. I could not easily find a viewer for windows, so I used [this web viewer](http://www.cs.rhodes.edu/welshc/COMP141_F16/ppmReader.html) hosted by Rhodes College

## "Benchmark"

I put this in quotes since I just ran a simple test once and with a randomly generated scene that was different for each program (But the generation code was the same) and did not analyze why they were different, so this means absolutely nothing. When I ran both the canonical C++ implementation (Outputting a ppm to stdout, compield using msvc 19.28.29337 for release) and this rust one (Using the default features) to generate the final scene for the book (500 samples per pixel, 1200 height, 3/2 aspect ratio, 50 depth, .1 aperture and 10 distance to focus) I got these numbers from powershell's Measure-Command:

Rust:

```
TotalHours        : 1.26086373911111
```

C++:

```
TotalHours        : 1.37428346177778
```

Rust with `--no-default-features --features wincrypt_rand,parallel` (I have an AMD 4800H with 8 physical and 16 logical cores):

```
TotalHours        : 0.0750414519722222
TotalMinutes      : 4.50248711833333
```

Same but with no blur (0.0 aperture and 1.0 distance to focus) and twice as many samples per pixel (1000):
(This is the image above)

```
TotalHours        : 0.159083646222222
TotalMinutes      : 9.54501877333333
```

## Things I might add in the future if I feel like it

- Command line parameters to select a scene, configure the camera, and configure the output image/aspect ratio/multisampling
- Ability to output more common image formats (png, jpeg, etc)

## Final image with blur

The image above has no depth of field, I thought it looked better. here's the one with blur:

![Same image as bove with depth of field, except a slightly different scene because it is randomly generated](/weekend_final.png)

## Other pretty renders

![](/moons.png)

This one is based on [this](https://github.com/POMMI3R/dasom-rs/tree/master/examples/small_balls):
![](/pastel.png)
