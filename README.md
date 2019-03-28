
# doom-fire-interop

<p align="center">
  <img alt="gif-of-doom-fire-on-browser" src="https://s3.amazonaws.com/media-p.slid.es/uploads/852577/images/5935155/chrome-capture.gif" width="490">
</p>

An implementation of the Doom Fire in Rust exported via FFI, with three apps (Web, iOS, Android) that use it. I actually did this project for a Rust LATAM talk.

This project is not intended to be the best way to create a Rust library that can compile to `Android`, `iOS` and `WebAssembly`.

It was created with the idea to share something that is not a hello world example of FFI that compiles to more than one platform.

It also contains a lot of the knowledge that me and [Philipe Paixão (aka @piiih)](https://github.com/piiih) acquired on building a project at [Pagar.me (@pagarme)](https://github.com/pagarme) that uses the same technology and ideas, that unfortunally because of regulamentation issues, needs to be closed source.


## Project Structure

```
doom-fire-interop
│
└───src
│   └─── all Rust code implementing the Doom Fire algorithm
│
└───DoomFireAndroid
│   └─── android app that consumes Rust library via FFI
│
└───DoomFireIOS
│   └─── ios app that consumes Rust library via FFI
│
└───DoomFireWeb
    └─── web app that consumes Rust library via WebAssembly
```

## How it works?

The idea is that the Rust library has a `PixelBoard` struct that contains a `Vec` of bytes (`u8`).
Each of those values represents the intensity of the fire for each of the pixels on the screen. Following this color pallette:

![color-pallete-image](https://user-images.githubusercontent.com/15306309/55189285-43fd9e80-517c-11e9-896b-41de3db3f579.png)

Then for each iteration of an infinity loop, an algorithm traverses all pixels and calculates their fire intensity based of: the pixel below, a "wind", and a random number.

## Example of usage

The code below is the implementation needed to actually run the Rust code (actually WebAssembly that was compiled from Rust) on JavaScript and render it on screen.
```javascript
import * as doom from 'doom'

function start() {
  const pixelBoard = doom.create_board(60, 40)

  doom.create_fire_source(pixelBoard) // sets the first line of pixels to 36

  setInterval(() => doom.calculate_fire_propagation(pixelBoard, renderFire), 50)
}

function renderFire(pixelsArray) {
  // pixelsArray = [35, 36, 22, 26, ...]
  // ... do rendering on screen
}

start()
```

## Where this idea came from?

There is a whole book about tricks that the Developers of the `Doom` game engine did, and there is an article, by the same author, just about the menu's fire:

http://fabiensanglard.net/doom_fire_psx/

Also, [@filipedeschamps](https://github.com/filipedeschamps) did a whole video about this, trying to simplify the algorithm:

https://www.youtube.com/watch?v=HCjDjsHPOco

The video is in portuguese, but it has subtitles :slightly_smiling_face:

He also has a repository containing a LOT of implementations of this algorithm. Some of them actually made this project possible.

[https://github.com/filipedeschamps/doom-fire-algorithm](https://github.com/filipedeschamps/doom-fire-algorithm)

## How to compile and run it?

### Android

Well, it isn't actually that simple, I recommend on reading this article by Mozilla that explains how it works to compile a Rust library and use it on an Android app: https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-21-rust-on-android.html.

### iOS

Same as Android, it requires a lot of configuration, here is a tutorial by Mozilla: https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-06-rust-on-ios.html.

### Web

This one is actually simpler :grin:

1. Install these tools: https://rustwasm.github.io/book/game-of-life/setup.html;
2. Clone this project;
3. Change directory to its folder (aka `cd doom-fire-interop`);
4. Run `wasm-pack build`;
5. Change to the new `pkg` folder created by `wasm-pack` (`cd pkg`);
6. Run `npm link`;
7. Change to the `DoomFireWeb` folder (`cd ../DoomFireWeb`);
8. Run `npm install`;
9. Run `npm link doom`;
10. Run `npm start`;

Then you should be able to see this at this URL: `http://localhost:8080`:

![browser-doom-fire-print](https://user-images.githubusercontent.com/15306309/55191114-19ade000-5180-11e9-80b2-7a268672e363.png)

## Thanks

<p align="center">
	<img alt="marcela-ziliotto-photo" src="https://github.com/marcelaziliotto.png" width="115" />
</p>

> [Marcela Ziliotto](https://github.com/marcelaziliotto) for lending her 2011's Macbook so that I could create the iOS app. :computer: :apple:

<p align="center">
	<img alt="philipe-paixao-photo" src="https://github.com/piiih.png" width="115" />
</p>

> [Philipe Paixão](https://github.com/piiih) for working with me on the Pagar.me project that lead to all the knowledge I know about WebAssembly and FFI. :man_technologist: :man_technologist:

<p align="center">
	<img alt="deivis-photo" src="https://github.com/deivis.png" width="115" />
	<img alt="rodrigo-melo-photo" src="https://github.com/rsmelo.png" width="115" />
	<img alt="mateus-moog-photo" src="https://github.com/moog.png" width="115" />
	<img alt="leonam-pereira-photo" src="https://github.com/leonampd.png" width="115" />
	<img alt="allan-jorge-photo" src="https://github.com/hails.png" width="115" />
	<img alt="pagarme-logo" src="https://github.com/pagarme.png" width="115" />
</p>

> [Deivis Windgert](https://github.com/deivis), [Rodrigo Melo](https://github.com/rsmelo), [Mateus Moog](https://github.com/moog), [Leonam Pereira](https://github.com/leonampd), [Allan Jorge](https://github.com/hails) and [Pagar.me](https://github.com/pagarme) for letting me practice my Rust LATAM presentation to them and giving me feedback. :speech_balloon:

<p align="center">
	<img alt="kassiano-photo" src="https://github.com/kassiano.png" width="115" />
	<img alt="murilo-da-paixao-photo" src="https://github.com/pogist.png" width="115" />
	<img alt="mateus-moog-photo" src="https://github.com/filipedeschamps.png" width="115" />
</p>

> [Kassiano](https://github.com/kassiano), [Murilo da Paixão](https://github.com/pogist) and [Filipe Deschamps](https://github.com/filipedeschamps) for putting your implementation open source so that I could actually know how to render pixels on screen. :globe_with_meridians: :video_game:


More people contributed to this, like my family and girlfriend, and other people at Pagar.me. Thank you all!! :heart:
