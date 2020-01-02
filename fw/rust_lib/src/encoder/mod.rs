pub struct Encoder {
    state: (bool, bool),
}

impl Encoder {
    pub const fn new() -> Encoder {
        return Encoder {
            state: (false, false),
        };
    }

    pub fn scan(&mut self, new_state: (bool, bool)) -> i32 {

        let mut change: i32 = 0;

        match self.state {
            (false, true) => {
                if new_state == (true, true) {
                    change = 1;
                }
                if new_state == (false, false) {
                    change = -1;
                }
            },

            (false, false) => {
                if new_state == (false, true) {
                    change = 1;
                }
                if new_state == (true, false) {
                    change = -1;
                }
            },

            (true, false) => {
                if new_state == (false, false) {
                    change = 1;
                }
                if new_state == (true, true) {
                    change = -1;
                }
            },

            (true, true) => {
                if new_state == (true, false) {
                    change = 1;
                }
                if new_state == (false, true) {
                    change = -1;
                }
            },
        };

        self.state = new_state;

        return change;
    }
}
