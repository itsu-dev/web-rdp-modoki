use std::{sync::Mutex, task::{Poll, Context}, pin::Pin, time::Duration, thread};

use actix_web::{web::{Bytes, Data}, Error};
use futures::{Stream, channel::mpsc::{channel, Receiver, Sender}};
use image::{codecs::jpeg::JpegEncoder, ColorType};
use screenshots::Screen;

pub struct Broadcaster {
    clients: Vec<Sender<Bytes>>,
}

impl Broadcaster {
    fn new() -> Self {
        Broadcaster {
            clients: Vec::new(),
        }
    }

    fn make_message_block(frame: &[u8], width: u32, height: u32) -> Vec<u8> {
        let mut buffer = Vec::new();
        let mut encoder = JpegEncoder::new_with_quality(&mut buffer, 50);
        encoder
            .encode(frame, width, height, ColorType::Rgb8)
            .unwrap();

        let mut msg = format!(
            "--boundarydonotcross\r\nContent-Length:{}\r\nContent-Type:image/jpeg\r\n\r\n",
            buffer.len()
        )
        .into_bytes();
        msg.extend(buffer);
        msg
    }

    fn send_image(&mut self, msg: &[u8]) {
        let mut ok_clients = Vec::new();
        let msg = Bytes::from([msg].concat());
        for client in self.clients.iter() {
            let result = client.clone().try_send(msg.clone());

            if let Ok(()) = result {
                ok_clients.push(client.clone());
            }
        }
        self.clients = ok_clients;
    }

    fn spawn_capture(me: Data<Mutex<Self>>) {
        let screens = Screen::all();
        let main_screen = screens.unwrap()[0];

        std::thread::spawn(move || loop {
            let pixels = main_screen.capture();
            let mut width = 0;
            let mut height = 0;

            let frame = match pixels {
                    Ok(mut pixels) => {
                        width = pixels.width() / 2;
                        height = pixels.height() / 2;
                        let mut buffer = vec![0; width as usize * height as usize * 3];

                        // RGBA to RGB
                        for x in 0..width {
                            for y in 0..height {
                                let pixel = pixels.get_pixel_mut(x * 2, y * 2);
                                let i = ((y * width + x) * 3) as usize;
                                buffer[i] = pixel[0];
                                buffer[i + 1] = pixel[1];
                                buffer[i + 2] = pixel[2];
                            }
                        }

                        buffer
                    }
                    _ => {
                        print!("failed to capture");
                        vec![0; width as usize * height as usize * 3]
                    }
                };

            let msg = Broadcaster::make_message_block(&frame, width, height);
            me.lock().unwrap().send_image(&msg);

            thread::sleep(Duration::from_millis((1000.0 / 25.0) as u64));
        });
    }

    pub fn create() -> Data<Mutex<Self>> {
        // Data ≃ Arc
        let me = Data::new(Mutex::new(Broadcaster::new()));

        Broadcaster::spawn_capture(me.clone());

        me
    }

    pub fn new_client(&mut self) -> Client {
        let (tx, rx) = channel(100);

        self.clients.push(tx);
        Client(rx)
    }
    
}

// wrap Receiver in own type, with correct error type
pub struct Client(Receiver<Bytes>);

impl Stream for Client {
    type Item = Result<Bytes, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.0).poll_next(cx) {
            Poll::Ready(Some(v)) => Poll::Ready(Some(Ok(v))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}