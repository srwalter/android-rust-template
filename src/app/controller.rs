use std::sync::mpsc::{channel, Sender, Receiver};

#[derive(Debug)]
enum IncomingMessage {
    Poke,
}

#[derive(Debug)]
pub enum OutgoingMessage {
    Toast(String)
}

pub struct Controller {
    tx: Sender<IncomingMessage>,
    rx: Receiver<OutgoingMessage>,
}

impl Controller {
    pub fn new () -> Self {
        let (ch1_tx, ch1_rx) = channel();
        let (ch2_tx, ch2_rx) = channel();

        std::thread::spawn(move || {
            Controller::thread(ch2_tx, ch1_rx);
        });

        Controller {
            tx: ch1_tx,
            rx: ch2_rx,
        }
    }
    
    pub fn try_recv(&mut self) -> Option<OutgoingMessage> {
        self.rx.try_recv().ok()
    }

    pub fn poke(&mut self) {
        self.tx.send(IncomingMessage::Poke).expect("failed to send poke");
    }

    fn thread(tx: Sender<OutgoingMessage>, rx: Receiver<IncomingMessage>) {
        loop {
            let msg = rx.recv();
            println!("msg {:?}", msg);
            tx.send(OutgoingMessage::Toast("Hello".to_string())).expect("failed to send toast");
        }
    }
}
