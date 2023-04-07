import {useState} from 'react'
import {listen} from '@tauri-apps/api/event'

function App() {
    const [message, setMessage] = useState("example message");

    const fn = async () => {
        await listen('send_message', (event) => {
            // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
            // event.payload is the payload object
            setMessage(event.payload as string);
        })
    };

    fn().catch(e => console.error(e));
    return (
        <div className="flex bg-white justify-center align-center items-center h-screen">
            <h1 className="text-7xl text-black font-sans text-center">{message}</h1>
        </div>
    )
}

export default App
