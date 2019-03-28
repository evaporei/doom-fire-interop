package com.otaviopace.doomfireandroid

import android.graphics.Color
import android.os.Bundle
import android.support.v7.app.AppCompatActivity
import android.view.Gravity
import android.widget.LinearLayout
import android.widget.TextView
import kotlinx.android.synthetic.main.activity_main.*
import org.jetbrains.anko.doAsync
import org.jetbrains.anko.uiThread
import java.util.*
import java.nio.ByteBuffer


class MainActivity : AppCompatActivity() {
    val fireWidth = 50
    val fireHeight = 50
    val numberOfPixels = fireWidth * fireHeight

    private val doomFire = DoomFire(fireWidth, fireHeight)

    val fireColorsPalette = arrayOf(
            Color.rgb(7,7,7),
            Color.rgb(31,7,7),
            Color.rgb(47,15,7),
            Color.rgb(71,15,7),
            Color.rgb(87,23,7),
            Color.rgb(103,31,7),
            Color.rgb(119,31,7),
            Color.rgb(143,39,7),
            Color.rgb(159,47,7),
            Color.rgb(175,63,7),
            Color.rgb(191,71,7),
            Color.rgb(199,71,7),
            Color.rgb(223,79,7),
            Color.rgb(223,87,7),
            Color.rgb(223,87,7),
            Color.rgb(215,95,7),
            Color.rgb(215,95,7),
            Color.rgb(215,95,7),
            Color.rgb(215,103,15),
            Color.rgb(207,111,15),
            Color.rgb(207,119,15),
            Color.rgb(207,127,15),
            Color.rgb(207,135,23),
            Color.rgb(199,135,23),
            Color.rgb(199,143,23),
            Color.rgb(199,151,31),
            Color.rgb(191,159,31),
            Color.rgb(191,159,31),
            Color.rgb(191,167,39),
            Color.rgb(191,167,39),
            Color.rgb(191,175,47),
            Color.rgb(183,175,47),
            Color.rgb(183,183,47),
            Color.rgb(183,183,55),
            Color.rgb(207,207,111),
            Color.rgb(223,223,159),
            Color.rgb(239,239,199),
            Color.rgb(255,255,255)
    )

    val fireTextView  by lazy {
        Array<TextView>(numberOfPixels) { createTextView() }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        doomFire.createFireSource()

        createInitialCanvas()

        renderFire(ByteBuffer.wrap(doomFire.getPixels()))

        Timer().scheduleAtFixedRate(createTask(),100, 250)
    }

    fun renderFire(firePixels: ByteBuffer){
        for(row in 0 until fireHeight){
            for(column in 0 until fireWidth){
                val pixelIndex =  column + (fireWidth * row)

                val tile = fireTextView[pixelIndex]

                tile.setBackgroundColor(fireColorsPalette[firePixels[pixelIndex].toInt()])
            }
        }
    }

    fun createInitialCanvas(){
        for(row in 0 until fireHeight){
            val linearRow = createLinearRow()

            for(column in 0 until fireWidth){
                val pixelIndex =  column + ( fireWidth * row )

                val tile = fireTextView[pixelIndex]

                linearRow.addView(tile)
            }

            fireCanvas.addView(linearRow)
        }
    }

    fun createTextView():TextView =
            TextView(this).apply {
                layoutParams = LinearLayout.LayoutParams(10,10)
                gravity = Gravity.CENTER
                textSize = 8F
            }

    fun createLinearRow(): LinearLayout =
            LinearLayout(this).apply {
                orientation = LinearLayout.HORIZONTAL
                layoutParams = LinearLayout
                        .LayoutParams(LinearLayout.LayoutParams.MATCH_PARENT,
                                LinearLayout.LayoutParams.WRAP_CONTENT)
            }

    // is executed forever  to propagate fire
    fun createTask() = object:TimerTask() {
        override fun run() {
            doAsync {
                uiThread {
                    doomFire.calculateFirePropagation { pixels -> renderFire(pixels) }
                }
            }
        }
    }
}
