pub mod messenger;
use messenger::Messenger;
use std::io;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

enum MESSAGE {
    Ping(u64),
    Pong(u64),
    Exit,
}

fn main() {
    let (a, b) = Messenger::<MESSAGE>::new();

    let (killer_send, killer_receive) = channel::<()>();

    let thread_one = thread::spawn(move || {
        let messager = a;
        let killer = killer_receive;

        match messager.send(MESSAGE::Ping(0u64)) {
            Ok(_) => {}

            Err(_) => {
                panic!("THREAD ONE COULD NOT SEND MESSAGE")
            }
        }

        loop {
            thread::sleep(Duration::from_millis(2500));

            match killer.try_recv() {
                Ok(_) => {
                    messager
                        .send(MESSAGE::Exit)
                        .expect("UNABLE TO EXIT CLEANLY, PANICING");
                    break;
                }

                _ => {}
            }

            match messager.receive() {
                Ok(m) => match m {
                    MESSAGE::Ping(l) => {
                        println!("THREAD ONE RECEIVED PING {}", l);

                        messager
                            .send(MESSAGE::Pong(l + 1))
                            .expect("MESSAGE SEND FAILED");
                    }

                    MESSAGE::Pong(l) => {
                        println!("THREAD ONE RECEIVED PONG {}", l);

                        messager
                            .send(MESSAGE::Ping(l + 1))
                            .expect("MESSAGE SEND FAILED");
                    }

                    _ => {}
                },
                Err(_) => panic!("THREAD ONE FAILED TO RECEIVE MESSAGE"),
            }
        }
    });

    let thread_two = thread::spawn(move || {
        let messager = b;

        loop {
            thread::sleep(Duration::from_millis(2500));

            match messager.receive() {
                Ok(m) => match m {
                    MESSAGE::Ping(l) => {
                        println!("THREAD TWO RECEIVED PING {}", l);

                        messager
                            .send(MESSAGE::Pong(l + 1))
                            .expect("MESSAGE SEND FAILED");
                    }

                    MESSAGE::Pong(l) => {
                        println!("THREAD TWO RECEIVED PONG {}", l);
                        messager
                            .send(MESSAGE::Ping(l + 1))
                            .expect("MESSAGE SEND FAILED");
                    }

                    MESSAGE::Exit => {
                        break;
                    }
                },
                Err(_) => panic!("THREAD TWO FAILED TO RECEIVE MESSAGE"),
            }
        }
    });

    let mut msg = String::new();

    io::stdin()
        .read_line(&mut msg)
        .expect("FAILED TO READ STDIN");

    killer_send
        .send(())
        .expect("UNABLE TO EXIT CLEANLY, PANICING");

    thread_one.join().expect("UNABLE TO JOIN THREAD ONE");
    thread_two.join().expect("UNABLE TO JOIN THREAD TWO");
}
