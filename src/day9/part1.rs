use std::thread;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct StreamResults {
    num_groups: u32,
    groups_score: u32,
}

/// Processes the stream of characters while updating results.
fn process_stream(stream: mpsc::Receiver<char>, results: &Mutex<StreamResults>) { 
    // processing is state dependent
    // compare values by copy, not reference
    #[derive(Copy, Clone)] enum Group { Nested(u32) }
    #[derive(Copy, Clone)] enum Garbage { Out, In }
    #[derive(Copy, Clone)] enum Ignore { Yes, No }
    let mut group_state = Group::Nested(0);
    let mut garbage_state = Garbage::Out;
    let mut ignore_state = Ignore::No;
    loop {
        match stream.recv() {
            Ok(c) => match (ignore_state, garbage_state, group_state, c) {
                // char following '!' is ignored, regardless of other state
                (Ignore::Yes, _, _, _) => ignore_state = Ignore::No,
                (Ignore::No, _, _, '!') => ignore_state = Ignore::Yes,

                // if in garbage, ignore all chars except '>' (and '!' above)
                // garbage starts on '<'
                (Ignore::No, Garbage::In, _, '>') => garbage_state = Garbage::Out,
                (Ignore::No, Garbage::In, _, _) => (),
                (Ignore::No, Garbage::Out, _, '<') => garbage_state = Garbage::In,

                // group starts on '{' and end on '}'
                (Ignore::No, Garbage::Out, Group::Nested(n), '{') => group_state = Group::Nested(n + 1),
                (Ignore::No, Garbage::Out, Group::Nested(n), '}') => {
                    group_state = Group::Nested(n - 1);
                    let mut r = results.lock().unwrap();
                    r.num_groups += 1;
                    r.groups_score += n;
                },
                (Ignore::No, Garbage::Out, Group::Nested(_), _) => (),
            }
            Err(_) => break
        }
    }
}

fn process(input: &str) -> StreamResults {
    // setup thread communication
    // arc - both main thread and worker thread need to own the data
    // mutex - allows thread safe, mutabe access
    let results = Arc::new(Mutex::new(StreamResults {
        num_groups: 0,
        groups_score: 0,
    }));
    let (tx, rx) = mpsc::channel();
    
    // spawn stream processor
    let shared_results = Arc::clone(&results);
    let processor = thread::spawn(move || {
        process_stream(rx, &shared_results);
    });

    // send chars to processor, wait for the result, then return it
    for c in input.chars() {
        tx.send(c);
    }

    // close the channel and wait for worker thread to complete
    drop(tx);
    processor.join();

    // wizardly to extract the results
    Arc::try_unwrap(results).unwrap().into_inner().unwrap()
}

#[test]
fn examples() {
    let garbage_1 = "<>";
    let garbage_2 = "<random characters>";
    let garbage_3 = "<<<<>";
    let garbage_4 = "<{!>}>";
    let garbage_5 = "<!!>";
    let garbage_6 = "<!!!>>";
    let garbage_7 = "<{o\"i!a,<{i<a>";
    assert_eq!(process(garbage_1).num_groups, 0);
    assert_eq!(process(garbage_2).num_groups, 0);
    assert_eq!(process(garbage_3).num_groups, 0);
    assert_eq!(process(garbage_4).num_groups, 0);
    assert_eq!(process(garbage_5).num_groups, 0);
    assert_eq!(process(garbage_6).num_groups, 0);
    assert_eq!(process(garbage_7).num_groups, 0);

    let group_1 = "{}";
    let group_2 = "{{{}}}";
    let group_3 = "{{},{}}";
    let group_4 = "{{{},{},{{}}}}";
    let group_5 = "{<{},{},{{}}>}";
    let group_6 = "{<a>,<a>,<a>,<a>}";
    let group_7 = "{{<a>},{<a>},{<a>},{<a>}}";
    let group_8 = "{{<!>},{<!>},{<!>},{<a>}}";
    assert_eq!(process(group_1).num_groups, 1);
    assert_eq!(process(group_2).num_groups, 3);
    assert_eq!(process(group_3).num_groups, 3);
    assert_eq!(process(group_4).num_groups, 6);
    assert_eq!(process(group_5).num_groups, 1);
    assert_eq!(process(group_6).num_groups, 1);
    assert_eq!(process(group_7).num_groups, 5);
    assert_eq!(process(group_8).num_groups, 2);

    let score_1 = "{}";
    let score_2 = "{{{}}}";
    let score_3 = "{{},{}}";
    let score_4 = "{{{},{},{{}}}}";
    let score_5 = "{<a>,<a>,<a>,<a>}";
    let score_6 = "{{<ab>},{<ab>},{<ab>},{<ab>}}";
    let score_7 = "{{<!!>},{<!!>},{<!!>},{<!!>}}";
    let score_8 = "{{<a!>},{<a!>},{<a!>},{<ab>}}";
    assert_eq!(process(score_1).groups_score, 1);
    assert_eq!(process(score_2).groups_score, 6);
    assert_eq!(process(score_3).groups_score, 5);
    assert_eq!(process(score_4).groups_score, 16);
    assert_eq!(process(score_5).groups_score, 1);
    assert_eq!(process(score_6).groups_score, 9);
    assert_eq!(process(score_7).groups_score, 9);
    assert_eq!(process(score_8).groups_score, 3);
}

fn main() {
    let input = include_str!("question");
    let r = process(input);
    println!("groups [{}] score [{}]", r.num_groups, r.groups_score);
}