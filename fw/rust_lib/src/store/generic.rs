use crate::os::{MailReceiver, MailSender};
use core::marker::PhantomData;

pub trait StoreState<Action> {
    fn handle_event(&mut self, action: Action, sender: &MailSender<Action>);
}

pub struct Store<Action, State, Listener>
where
    State: StoreState<Action>,
    Listener: FnMut(&State),
{
    phantom_action: PhantomData<Action>,
    state: State,
    listener: Listener,
}

impl<A, S, L> Store<A, S, L>
where
    S: StoreState<A>,
    L: FnMut(&S),
{
    pub fn new(initial_state: S, listener: L) -> Store<A, S, L> {
        Store {
            phantom_action: PhantomData,
            state: initial_state,
            listener,
        }
    }

    pub fn force_update(&mut self) {
        (self.listener)(&self.state);
    }

    pub fn handle_events(&mut self, sender: MailSender<A>, receiver: MailReceiver<A>) -> ! {
        loop {
            let event = receiver.recv().unwrap();
            self.state.handle_event(event, &sender);
            (self.listener)(&self.state);
        }
    }
}
