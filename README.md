# Bitwig Studio NLI

Fancy tool which will use a extension to communicate MIDI info from Bitwig
over to the Server, which then can display that info on a website to be able to be used
on an OBS scene as an example.

## Structure

Describes seperation of Kotlin side (ran though Bitwig's process) and the Rust side,
which is our own process, which can we more dynamic, and be crashed, without Bitwig

### Kotlin Handling

The Kotlin side of this will be kept to a minimum, to appoint all heavy-handling
over to the Rust, which will allow for Bitwig to have least of a chance for us
to crash it. So all we will do is:
* Grab controller information
* Setup a WS client to spell that to the WS server

### Rust Handling

The Rust handing is pretty much everything you might think this program actually needs to run:

* Getting the MIDI info
    - Bitwig Extension side does the "real" getting
    - We host a Server which the Extension sends their data to this
    - Fill out all information in types which then the Rust code can use
      without the nonsense of JSON, or remenance of ECMAScript.
* Host a website which can be used to find, and display the device
* WS Sockets to discuss the state of the device, automatically

## License

All current code is licensed under Apache-2.0.
