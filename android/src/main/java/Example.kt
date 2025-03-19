package com.plugin.os

import android.app.Activity
import android.content.SharedPreferences
import androidx.security.crypto.EncryptedSharedPreferences
import androidx.security.crypto.MasterKeys

class Example(private val activity: Activity) {
    private val prefs: SharedPreferences = EncryptedSharedPreferences.create(
        "secret_shared_prefs",
        MasterKeys.getOrCreate(MasterKeys.AES256_GCM_SPEC),
        this.activity,
        EncryptedSharedPreferences.PrefKeyEncryptionScheme.AES256_SIV,
        EncryptedSharedPreferences.PrefValueEncryptionScheme.AES256_GCM
    )

    fun store(key: String, value: String) {
        val editor = this.prefs.edit()
        editor.putString(key, value)
        editor.apply()
    }

    fun load(key: String): String? {
        return this.prefs.getString(key, "DEFAULT")
    }
}
