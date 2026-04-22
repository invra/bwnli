import io.ktor.client.*
import io.ktor.client.engine.cio.*
import io.ktor.client.plugins.websocket.*
import io.ktor.websocket.*
import kotlinx.serialization.Serializable
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json

@Serializable
data class Message(
    val type: String,
    val payload: String,
    val timestamp: Long = System.currentTimeMillis()
)

suspend fun main() {
    val client = HttpClient(CIO) {
        install(WebSockets)
    }

    try {
        client.webSocket(host = "localhost", port = 55553, path = "/ws") {
            val message = Message(type = "greeting", payload = "Hello from Kotlin!")
            val json = Json.encodeToString(message)

            send(Frame.Text(json))
            println("Sent: $json")

            for (frame in incoming) {
                if (frame is Frame.Text) {
                    println("Received: ${frame.readText()}")
                }
            }
        }
    } catch (e: java.net.ConnectException){
        println("Couldn't connect to the Websockets server!")
    } catch (e: Exception) {
        println("Unhandled exception: ${e.localizedMessage}")
    } finally {
        client.close()
    }
}
