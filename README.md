# rFactor 2 Event Marker
This is a small CLI application to create event markers for rFactor 2.  
It was written for the GSMC Apex Cup, to be used by our broadcaster to quickly be able to react to events and capture them using a Stream Deck.

## Usage with a Stream Deck
Create a "Open Application" button and navigate to the executable.  
It should look something like:  

`"<path to .exe>" --url "<url to broadcast control panel>"`

Be careful to not have a trailing slash in the URL.

## Optional flags
You can set the offset at which the event marker should be placed:  
`--offset=<offset in seconds>`

You can also set the default camera to use:  
`--camera-index=<camera index>`
