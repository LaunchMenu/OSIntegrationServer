# Planning

## Client implementation strategy

## Server implementation strategy


The following would be an implementation of what the server would like to achieve:

```ts
import Features from "somewhere"       //Type Features = {[key: string]: IFeature}
import MessageServer from "somewhere"  //{send(msg: Message, expectResponse: boolean), sendUnhandled(msg: Message)}
import Win32 from "somewhere"
/**
 * main
 * features
 * |- KeyServer
 * |- AutomationServer
 * |- AudioServer
 */
const mainQueue: ItemQueue<Message> = [];
function main(){
    new Thread(()=>{
        while(true) {
            let msg = mainQueue.pop();
            if(Features.exists(msg.kind)){
                let feature = Features[msg.kind];
                feature.queue.push(msg);
            } else {
                MessageServer.raiseUnhandled(msg);
            }
        }
    })
    while(true) {
        let msg = await MessageFromIO();
        mainQueue.push(msg);
    }
}

//ServerSide setup
class BaseFeature: IFeature {
    public queue: ItemQueue<Message>                                 //ThreadSafe
    protected kind: string = "00000000-0000-0000-0000-000000000000"; //ThreadSafe
    public main(){                                                   //Spawns thread
        new Thread(()=>{
            while(true) {
                MessageServer.sendUnhandled(queue.pop());
            }
        })
    }
}
class KeyServer extends BaseFeature {
    protected kind: string = "a8abdfdc-fcce-4543-878d-0f9dddfec445"
    public main(){
        new Thread(()=>{
            //Hook windows loop
            Win32.setWinEventHook(proc)

            while(true) {
                let msg = queue.pop()
                switch(msg.data[0]){
                    case "shortcut-register":
                        shortcuts[msg.data[1]] = msg.data[2]
                        break;
                    case "shortcut-delete":
                        delete shortcuts[msg.data[1]]
                        break
                    default:
                        MessageServer.sendUnhandled(msg);
                }
            }
        })
    }
    public proc(hwnd: Win32.hwnd, message: int, lparam: LParam, wparam: WParam){
        let capture: boolean = false
        if shortcuts[key]{
            let response = createMessage("shortcut", ...)
            MessageServer.send(response, false)
            capture=true
        }
        let resp = await MessageServer.send("keyPress", response, expectResponse: true)
        if (resp) capture = true
        if (capture){
             return 0
        } else {
            return Win32.callNextHook(window,message,lparam,wparam)
        }
    }
}
```