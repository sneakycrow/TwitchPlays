import {useState} from 'react'
import SVG from 'react-inlinesvg';
import './App.css'
import {listen} from '@tauri-apps/api/event'

function App() {
    const [count, setCount] = useState(0)
    const [message, setMessage] = useState("");

    const fn = async () => {
        await listen('send_message', (event) => {
            // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
            // event.payload is the payload object
            setMessage(event.payload as string);
        })
    };

    fn().catch(e => console.error(e));
    return (
        <div className="wrapper">
            <SVG
                src="/controller.svg"
                width={1200}
                height="auto"
                title="React"
                className="controller"
            />
            <p>{message}</p>
        </div>
    )
}

export default App
