# DoomFireIOS

Well, I don't actually knew how to render pixels on screen on iOS, since I don't usually do iOS development, and I don't have a Mac :apple:.

So I found one implementation online that helped me a LOT! Which is this one: [https://github.com/filipedeschamps/doom-fire-algorithm/tree/master/playground/doom-fire-algorithm-ios-spritekit](https://github.com/filipedeschamps/doom-fire-algorithm/tree/master/playground/doom-fire-algorithm-ios-spritekit).

The difference between this project and the other one is that the core Doom Fire logic is on Rust. Also that since I had an old Mac, I had to do this in Swift 2.2, so the `SpriteKit` API was a bit different.

So yeah, this project uses `SpriteKit` to render things on screen.
