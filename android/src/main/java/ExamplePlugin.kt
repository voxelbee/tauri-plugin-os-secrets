package com.plugin.os
import android.app.Activity
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin

@InvokeArg
class SetRequest {
    var key: String? = null
    var value: String? = null
}

@InvokeArg
class GetRequest {
    var key: String? = null
}

@InvokeArg
class RemoveRequest {
    var key: String? = null
}

@TauriPlugin
class ExamplePlugin(private val activity: Activity): Plugin(activity) {
    private val implementation = Example(activity)

    @Command
    fun set(invoke: Invoke) {
        val args = invoke.parseArgs(SetRequest::class.java)

        implementation.set(args.key ?: "default key :(", args.value ?: "default value :(")
        invoke.resolve()
    }

    @Command
    fun get(invoke: Invoke) {
        val args = invoke.parseArgs(GetRequest::class.java)

        val ret = JSObject()
        ret.put("value", implementation.get(args.key ?: "default value :("))
        invoke.resolve(ret)
    }

    @Command
    fun remove(invoke: Invoke) {
        val args = invoke.parseArgs(RemoveRequest::class.java)

        implementation.remove(args.key ?: "default value :(")
        invoke.resolve()
    }
}
