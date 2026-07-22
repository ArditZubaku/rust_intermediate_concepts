use crossbeam::channel::{self, Receiver, Sender};
use std::{thread, time::Duration};

#[derive(Debug)]
enum Lunch {
    Soup,
    Salad,
    Sandwich,
    HotDog,
}

/*
  THREAD & CHANNEL ARCHITECTURE:
  =============================

                      +-----------------------+
                      |      MAIN THREAD      |
                      |   Sends orders &      |
                      |   receives lunches    |
                      +-----------+-----------+
                                  |
                                  |  [orders_tx]
                                  v
  ................................|...................................
  CHANNEL: orders  [ unbounded queue for &str ]
  ................................|...................................
                                  |
                     +------------+------------+
                     |                         |
                     v [orders_rx2]            v [orders_rx]
            +-----------------+       +-----------------+
            |  ALICE THREAD   |       |   ZACK THREAD   |
            | (Worker 1)      |       | (Worker 2)      |
            +--------+--------+       +--------+--------+
                     |                         |
                     v [lunches_tx2]           v [lunches_tx]
  ...................|.........................|......................
  CHANNEL: lunches   [ unbounded queue for Lunch ]
  ...................|................................................
                     |
                     v  [lunches_rx]

                      +-----------------------+
                      |      MAIN THREAD      |
                      |   Prints final Lunch  |
                      +-----------------------+
*/

fn main() {
    let (orders_tx, orders_rx) = channel::unbounded::<&str>();
    let orders_rx2 = orders_rx.clone();

    let (lunches_tx, lunches_rx) = channel::unbounded::<Lunch>();
    let lunches_tx2 = lunches_tx.clone();

    let alice_handle = thread::spawn(|| cafeteria_worker("alice", orders_rx2, lunches_tx2));
    let zack_handle = thread::spawn(|| cafeteria_worker("zack", orders_rx, lunches_tx));

    for order in [
        "polish dog",
        "caesar salad",
        "onion soup",
        "reuben sandwich",
    ] {
        println!("ORDER: {}", order);
        let _ = orders_tx.send(order);
    }
    drop(orders_tx);

    for lunch in lunches_rx {
        println!("Order Up! -> {:?}", lunch);
    }

    let _ = alice_handle.join();
    let _ = zack_handle.join();
}

fn cafeteria_worker(name: &str, orders: Receiver<&str>, lunches: Sender<Lunch>) {
    for order in orders {
        println!("{} receives an order for {}", name, order);

        let lunch = match &order {
            x if x.contains("soup") => Lunch::Soup,
            x if x.contains("salad") => Lunch::Salad,
            x if x.contains("sandwich") => Lunch::Sandwich,
            _ => Lunch::HotDog,
        };

        for _ in 0..order.len() {
            thread::sleep(Duration::from_secs_f32(0.1));
        }

        println!("{} sends a {:?}", name, lunch);

        if lunches.send(lunch).is_err() {
            break;
        }
    }
}
