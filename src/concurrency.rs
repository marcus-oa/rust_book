use std::thread;
use std::time::Duration;

pub fn thread_example_one() {
    // spawns a single thread for use
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("THREAD EXAMPLE ONE:hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // uses the main thread as we haven't spawned a thread
    // for usage
    for i in 1..5 {
        println!("THREAD EXAMPLE ONE: hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // as type of handle variable is JoinHandle
    // and returns result when join is called on it.
    // Join waits for the thread to finish thus ensuring handle
    // finishes before we exist the main
    handle.join().unwrap()
}

pub fn thread_example_two() {
    // spawns a single thread for use
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("THREAD EXAMPLE TWO: hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // placing the call to join earlier blocks any other threads
    // from running until the handle thread finishes
    handle.join().unwrap();

    // uses the main thread as we haven't spawned a thread
    // for usage
    for i in 1..5 {
        println!("THREAD EXAMPLE TWO: hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

pub fn move_thread_example() {
    let v = vec![1, 2, 3];

    // move means the thread takes ownership of v
    // such that the reference println requires for v
    // will definitely be in scope
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // This won't work after move applied as error dictates we're trying
    // to use a moved value and the ownership rules won't allow it
    // drop(v);
    handle.join().unwrap()
}

//-------------------------------------------------------------------------------------------
//---------------- ch16-02-message-passing ---------------------
//-------------------------------------------------------------------------------------------

use std::sync::{mpsc, Arc};

pub fn message_passing_example() {
    // creates a transaction sender and transaction receiver from
    // the channel function
    let (tx, rx) = mpsc::channel();

    // using a spawn thread we move the tx so it's owned by said
    // thread and send a message
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });


    // we receive the value sent in tx with the main thread
    // and print
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

// function to display the concurrent nature
// of the threads sending and receiving
pub fn message_passing_example_two() {

    // creates a transaction sender and transaction receiver from
    // the channel function
    let (tx, rx) = mpsc::channel();

    // using a spawn thread we move the tx so it's owned by said
    // thread and send a vector of messages
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),];

        for val in vals {
            tx.send(val).unwrap();
            // sleep between message send
            thread::sleep(Duration::from_secs(1));
        }
    });

    // no longer explicitly calling the recv function
    // and treating rx as an iterator of results
    for received in rx {
        println!("Got: {}", received);
    }
}

// example to display multiple tx sending messages in the channel
pub fn multi_message_sending_example() {

    let (tx, rx) = mpsc::channel();

    // cloned tx to use as one of multiple senders
    let tx1 = mpsc::Sender::clone(&tx);

    // using a spawn thread we move the tx so it's owned by said
    // thread and send a vector of messages
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),];

        for val in vals {
            tx1.send(val).unwrap();
            // sleep between message send
            thread::sleep(Duration::from_secs(1));
        }
    });

    // sending more transactions from the cloned tx
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),];

        for val in vals {
            tx.send(val).unwrap();
            // sleep between message send
            thread::sleep(Duration::from_secs(1));
        }
    });

    // no longer explicitly calling the recv function
    // and treating rx as an iterator of results
    for received in rx {
        println!("Got: {}", received);
    }
}

//-------------------------------------------------------------------------------------------
//---------------- ch16-03-shared-state ---------------------
//-------------------------------------------------------------------------------------------

use std::sync::Mutex;
use std::rc::Rc;

// example of using a mutual exclusive and a lock
// on the thread
pub fn mutex_example() {
    let m = Mutex::new(5);

    {
        // lock usage means we lock the thread to using this mutex
        // no other thread can use but seeing as this is a one thread main
        // theres no difference to using a mutex or not here (until
        // we introduce threads)
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

// example of sharing state across multiple threads using
// a mutual exclusive value we want to change and Arc
// (Atomic Reference counter) which allows multiple atomic references
// that work in thread safe ways when multiple threads access them
// Similiar to Rc but atomic!
pub fn shared_mutex_thread_example() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle)
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}