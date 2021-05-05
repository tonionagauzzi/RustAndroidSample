package com.vitantonio.nagauzzi.rustandroidsample

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import com.vitantonio.nagauzzi.rustandroidsample.databinding.ActivityMainBinding

class MainActivity : AppCompatActivity() {
    private lateinit var binding: ActivityMainBinding
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivityMainBinding.inflate(layoutInflater).apply { setContentView(this.root) }
        val gender = someGender()
        val name = "${someFirstName(gender)}.${someLastName()}"
        val birthday = someBirthday()
        val profile = "Name: $name\nGender: $gender\nBirthday: $birthday"
        binding.sampleText.text = "${getString(R.string.top_message)}\n\n${profile}"
    }

    private external fun someGender(): String
    private external fun someFirstName(gender: String): String
    private external fun someLastName(): String
    private external fun someBirthday(): String

    companion object {
        init {
            System.loadLibrary("hello")
        }
    }
}