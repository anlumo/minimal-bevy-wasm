# Minimal Bevy WASM Example

This demonstrates the minimal requirements for using bevy in a browser. It does not include any event handling and the canvas is hardcoded into the HTML file.

## Usage

Install [Trunk](https://trunkrs.dev/) and run

```
trunk serve
```

It should take care of the rest.

## Notes

I tried to minimize the wasm size using compiler and wasm-opt flags. Run `trunk build --release` still results in a 3.5MB wasm file for me, so I guess bevy is just pretty bloated, even when you remove all optional parts.

# License

minimal-bevy-wasm by Andreas Monitzer

To the extent possible under law, the person who associated CC0 with
minimal-bevy-wasm has waived all copyright and related or neighboring rights
to minimal-bevy-wasm.

You should have received a copy of the CC0 legalcode along with this
work.  If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
