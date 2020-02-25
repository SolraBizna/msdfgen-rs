/*!
# Bundled msdfgen library

This crate provides bundled [msdfgen](https://github.com/Chlumsky/msdfgen) library for using with [__msdfgen__](https://crates.io/crates/msdfgen) crate.

## Usage

You can simply add this as dependency to your manifest:

```toml
[dependencies]
msdfgen = "^0.1"

# Use bundled library to avoid unresolved links
msdfgen-lib = "^0.1"
```

Next you should say compiler that you want to use that crate:

```rust
// Either in traditional manner
extern crate msdfgen_lib;

// Or in Rust2018 manner
use msdfgen_lib as _;
```

## Features

You can apply some customizations to library using those features:

- __shared__ Force bundle shared (or dynamic) library instead of static
- __stdcxx__ Link with _libstdc++_ instead of _libc++_
- __stdcxx-shared__ Link with shared C++ stdlib instead of static

 */

#[cfg(test)]
mod test {
    #[repr(C)]
    struct msdfgen_SignedDistance {
        distance: f64,
        dot: f64,
    }

    extern "C" {
        #[link_name = "\u{1}_ZN7msdfgen14SignedDistance8INFINITEE"]
        static msdfgen_SignedDistance_INFINITE: msdfgen_SignedDistance;
    }

    #[test]
    fn linking() {
        let infinite = unsafe { &msdfgen_SignedDistance_INFINITE };

        assert_eq!(infinite.distance, -1000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0);
        assert_eq!(infinite.dot, 1.0);
    }
}
