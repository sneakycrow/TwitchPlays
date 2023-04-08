import {listen} from "@tauri-apps/api/event";
import {useState} from "react";

const Message = () => {
    const [message, setMessage] = useState("example message");
    const receiveMessage = async () => {
        await listen('send_message', (event) => {
            // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
            // event.payload is the payload object
            setMessage(event.payload as string);
        })
    };

    receiveMessage().catch(e => console.error(e));
    return (
        <div className="flex bg-white justify-center items-center border-8">
            <h1 className="text-7xl text-black font-sans text-center">{message}</h1>
        </div>
    )
}

export default Message;