# DoomFireAndroid

Well, I don't actually knew how to render pixels on screen on Android, since I don't usually do Android development.

So I found one implementation online that helped me a LOT! Which is this one: [https://github.com/filipedeschamps/doom-fire-algorithm/tree/master/playground/android-implementation-kotlin](https://github.com/filipedeschamps/doom-fire-algorithm/tree/master/playground/android-implementation-kotlin).

The difference between this project and the other one is that the core Doom Fire logic is on Rust. Also that the minimum API version for Android is 15, which correspond to the platform version `4.0.3`, `4.0.4`.

By the way this uses a Kotlin Android library called `Anko`.
