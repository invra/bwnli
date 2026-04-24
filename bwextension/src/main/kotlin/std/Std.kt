import java.util.concurrent.locks.ReentrantLock
import kotlin.concurrent.withLock
import com.bitwig.extension.controller.api.ControllerHost

class AppInstance private constructor(host: ControllerHost) {
    private val lock = ReentrantLock()
    private var data: ControllerHost = host

    fun set(value: ControllerHost) {
        lock.withLock { data = value }
    }

    fun get(): ControllerHost {
        lock.withLock { return data }
    }

    companion object {
        @Volatile
        private var instance: AppInstance? = null

        fun init(host: ControllerHost): AppInstance {
            return instance ?: synchronized(this) {
                instance ?: AppInstance(host).also { instance = it }
            }
        }

        fun getInstance(): AppInstance {
            return instance
                ?: error("AppInstance not initialized. Call AppInstance.init(host) first.")
        }
    }
}

object stdlib {
    fun notify(msg: String) {
        val host = AppInstance.getInstance().get()
        host.showPopupNotification(msg)
    }
}
