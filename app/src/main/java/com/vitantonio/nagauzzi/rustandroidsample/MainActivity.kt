package com.vitantonio.nagauzzi.rustandroidsample

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import com.vitantonio.nagauzzi.rustandroidsample.databinding.ActivityMainBinding

class MainActivity : AppCompatActivity() {
    private lateinit var binding: ActivityMainBinding
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivityMainBinding.inflate(layoutInflater).apply { setContentView(this.root) }
        binding.sampleText.text = stringFromJNI()
    }

    /**
     * A native method that is implemented by the 'native-lib' native library,
     * which is packaged with this application.
     */
    private external fun stringFromJNI(): String

    companion object {
        init {
            System.loadLibrary("hello")
        }
    }
}