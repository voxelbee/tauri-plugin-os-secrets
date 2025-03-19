package com.plugin.os
import android.app.Activity
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin

@InvokeArg
class StoreRequest {
    var key: String? = null
    var value: String? = null
}

@InvokeArg
class LoadRequest {
    var key: String? = null
}

@TauriPlugin
class ExamplePlugin(private val activity: Activity): Plugin(activity) {
    private val implementation = Example(activity)

    @Command
    fun store(invoke: Invoke) {
        val args = invoke.parseArgs(StoreRequest::class.java)

        implementation.store(args.key ?: "default key :(", args.value ?: "default value :(")
        invoke.resolve()
    }

    @Command
    fun load(invoke: Invoke) {
        val args = invoke.parseArgs(LoadRequest::class.java)

        val ret = JSObject()
        ret.put("value", implementation.load(args.key ?: "default value :("))
        invoke.resolve(ret)
    }
}
