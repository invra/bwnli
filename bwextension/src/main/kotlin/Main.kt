package com.invra

import com.bitwig.extension.controller.ControllerExtension
import com.bitwig.extension.controller.api.ControllerHost

class BwNLIExtension(definition: BwNLIExtensionDefinition, host: ControllerHost)
    : ControllerExtension(definition, host) {

    override fun init() {
        // Load in host to a singleton that stdlib will read.
        // stdlib is architechted like this so simple tasks like
        // `host.showPopupNotification(msg)` is just
        // `stdlib.notify(msg)`.
        AppInstance.init(host)
        stdlib.notify("Testing has loaded!")
    }

    override fun exit() {}
    override fun flush() {}
}
