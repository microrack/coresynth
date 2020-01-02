use super::{Closure, Duration, Instant, MailReceiver, MailSender, Mutex, mail_queue, spawn};
use crate::peripheral::Static;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct TimerId(u32);

#[derive(Debug)]
struct PendingTimer {
    id: TimerId,
    target: Instant,
    callback: Closure,
}

struct TimersState {
    next_id: u32,
}

impl TimersState {
    fn get_next_id(&mut self) -> TimerId {
        let result = self.next_id;
        self.next_id += 1;
        TimerId(result)
    }
}

struct PendingTimers {
    // TODO use heapless vec
    timers: [Option<PendingTimer>;7],
}

enum TimersInput {
    NewTimer(PendingTimer),
    CancelTimer(TimerId),
}

pub fn init_timers() {
    TIMERS_STATE.init(Mutex::new(TimersState {next_id: 0}).unwrap());

    let (timers_sender, timers_receiver) = mail_queue::<TimersInput>(10).unwrap();
    TIMERS_SENDER.init(timers_sender);
    TIMERS_RECEIVER.init(Mutex::new(timers_receiver).unwrap());

    spawn("timers_task", 256, timers_thread_fn).unwrap();
}

fn common_timeout(duration: Duration, callback: Closure) -> TimerId {
    let now = Instant::now();

    // debug_println!("add timeout {:?}, now {:?}", duration, now);

    let target = now + duration;

    let id = {
        TIMERS_STATE.lock().unwrap().get_next_id()
    };
    let _ = TIMERS_SENDER.send(TimersInput::NewTimer(PendingTimer {
        id,
        target,
        callback,
    }));
    id
}

pub fn timeout<F: FnMut() + Send>(duration: Duration, callback: F) -> TimerId {
    common_timeout(duration, Closure::new(callback))
}

pub fn cancel_timeout(id: TimerId) {
    let _ = TIMERS_SENDER.send(TimersInput::CancelTimer(id));
}

static TIMERS_STATE: Static<Mutex<TimersState>> = Static::new();
static TIMERS_SENDER: Static<MailSender<TimersInput>> = Static::new();
// TODO receiver should be moved completely to timers thread
static TIMERS_RECEIVER: Static<Mutex<MailReceiver<TimersInput>>> = Static::new();

extern "C" fn timers_thread_fn() {
    let mut timers = PendingTimers::new();
    let receiver = TIMERS_RECEIVER.lock().unwrap();

    loop {
        let now = Instant::now();
        let next_timeout = timers.next_timeout(now);

        match next_timeout {
            Some(timeout) => {
                // debug_println!("wait {:?}", timeout);

                match receiver.recv_timeout(timeout) {
                    Ok(event) => timers.handle_input(event),
                    Err(crate::os::RecvTimeoutError::Timeout) => {/*do nothing*/},
                    _ => panic!("recv_timeout unexpected error"),
                };
            },
            None => timers.handle_input(receiver.recv().unwrap()),
        };

        let now = Instant::now();
        timers.advance_timers(now);
    }
}

impl PendingTimers {
    fn new() -> PendingTimers {
        PendingTimers {
            timers: Default::default(),
        }
    }

    fn next_timeout(&self, now: Instant) -> Option<Duration> {
        self.timers.iter()
            .filter_map(|o| o.as_ref())
            .map(|o| o.target)
            .min()
            .map(|i| {
                if i <= now {
                    Duration::from_ms(0)
                } else {
                    (i - now).as_duration()
                }
            })
    }

    fn push(&mut self, timer: PendingTimer) {
        for i in 0 .. self.timers.len() {
            if self.timers[i].is_none() {
                self.timers[i] = Some(timer);
                return;
            }
        }
    }

    fn cancel(&mut self, id: TimerId) {
        for i in 0 .. self.timers.len() {
            if let Some(timer) = &self.timers[i] {
                if timer.id == id {
                    self.timers[i] = None;
                    return;
                }
            }
        }
    }

    fn handle_input(&mut self, event: TimersInput) {
        match event {
            TimersInput::NewTimer(timer) => self.push(timer),
            TimersInput::CancelTimer(id) => self.cancel(id),
        }
    }

    fn advance_timers(&mut self, now:Instant) {
        for i in 0 .. self.timers.len() {
            if let Some(timer) = &mut self.timers[i] {
                // debug_println!("target {:?}, now {:?}", timer.target, now);

                if timer.target <= now {
                    // debug_println!("call");
                    timer.callback.call();
                    self.timers[i] = None;
                }
            }
        }
    }
}
