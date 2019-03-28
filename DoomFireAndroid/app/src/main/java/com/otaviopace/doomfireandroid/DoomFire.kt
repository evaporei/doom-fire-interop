package com.otaviopace.doomfireandroid

import java.nio.ByteBuffer

class DoomFire(val width: Int, val height: Int) {
    // pixelBoard memory address
    private val pixelBoard: Long

    private var renderCallback: ((ByteBuffer) -> Unit)? = null

    private external fun createBoard(width: Int, height: Int): Long
    private external fun createFireSource(board: Long)
    private external fun calculateFirePropagation(board: Long)
    private external fun getPixels(board: Long): ByteArray
    private external fun freeBoard(board: Long)

    init
    {
        // Load Rust library
        System.loadLibrary("doom");

        pixelBoard = createBoard(width, height)
    }

    fun createFireSource() {
        createFireSource(pixelBoard)
    }

    fun calculateFirePropagation(render: (ByteBuffer) -> Unit) {
        renderCallback = renderCallback ?: render

        calculateFirePropagation(pixelBoard)// it will eventually call DoomFire.render
    }

    fun getPixels(): ByteArray {
        return getPixels(pixelBoard)
    }

    // calculateFirePropagation will call this function, like a callback
    fun render(data: ByteBuffer) {
        renderCallback!!(data)
    }

    fun freeResources() {
        freeBoard(pixelBoard)
    }
}