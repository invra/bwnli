package com.invra

import com.bitwig.extension.api.PlatformType
import com.bitwig.extension.controller.AutoDetectionMidiPortNamesList
import com.bitwig.extension.controller.ControllerExtensionDefinition
import com.bitwig.extension.controller.api.ControllerHost
import java.util.UUID

class BwNLIExtensionDefinition : ControllerExtensionDefinition() {
    override fun getName() = "Bitwig Novation Launchpad Information"
    override fun getAuthor() = "invra"
    override fun getVersion() = "0.1"
    override fun getId(): UUID = UUID.fromString("055c9556-76b8-400d-b204-2c45243fc1aa")
    override fun getHardwareVendor() = "invra"
    override fun getHardwareModel() = "BW-NLI"
    override fun getRequiredAPIVersion() = 25
    override fun getNumMidiInPorts() = 0
    override fun getNumMidiOutPorts() = 0

    override fun listAutoDetectionMidiPortNames(
        list: AutoDetectionMidiPortNamesList,
        platformType: PlatformType
    ) {}

    override fun createInstance(host: ControllerHost) = BwNLIExtension(this, host)
}
