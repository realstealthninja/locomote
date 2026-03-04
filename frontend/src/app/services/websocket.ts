import { Injectable } from '@angular/core';
import { Observable, Subject } from 'rxjs';

@Injectable({
  providedIn: 'root',
})
export class WebsocketService {
  private websocket: WebSocket;
  private messages: Subject<any> = new Subject();
  constructor() {
    this.connect();
  }
  private connect() {
    this.websocket = new WebSocket('ws://localhost:8080');
    
    this.websocket.onmessage = (event) => {
      this.messages.next(JSON.parse(event.data));
    };
    
    this.websocket.onerror = (error) => {
      console.error("WebSocket error:", error);
    };
    
    this.websocket.onclose = () => {
      console.log("WebSocket connection closed");
    };
  }
  sendMessage(message: any) {
    if (this.websocket.readyState === WebSocket.OPEN) {
      this.websocket.send(JSON.stringify(message));
    }
  }
  getMessages(): Observable<any> {
    return this.messages.asObservable();
  }
}
