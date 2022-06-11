pub enum Event {
    Click { mousex: usize, mousey: usize },
    KeyPress { keycode: usize },
}

pub struct EventSystem {
    listeners: Vec<Box<dyn FnMut(Event) -> ()>>,
}

impl EventSystem {
    pub fn add_listener(&mut self, f: impl FnMut(Event) -> () + 'static) {
        self.listeners.push(Box::new(f));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closed_event() {
        let mut events = EventSystem {
            listeners: Vec::new(),
        };
        events.add_listener(|e| {
            if let Event::Click { mousex, mousey } = e {
                println!("Got a click event at ({}, {})", mousex, mousey);
            }
        });

        events.add_listener(|e| {
            if let Event::KeyPress { keycode } = e {
                println!("Got a keypress {}", keycode);
            }
        });

        events.listeners[0](Event::Click {
            mousex: 100,
            mousey: 200,
        });
        events.listeners[1](Event::Click {
            mousex: 100,
            mousey: 200,
        });
        events.listeners[0](Event::KeyPress { keycode: 12345 });
        events.listeners[1](Event::KeyPress { keycode: 12345 });
    }
}
