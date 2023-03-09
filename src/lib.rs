//! # Limit Tracker
//! 
//! 'LimitTracker' is a simple demo library, used to measure amount of quota 
//! used, i.e. of API account usage.
//! It handles only the tracking of usage, and sends message via Messenger

/// Messenger is a user defined object that hadles sending messages to user
pub trait Messenger {
	fn send(&self, msg: &str);
} 

/// LimitTracker is used to track usage of some resource.
/// Messenger is a user defined object implementing the Messenger trait.
pub struct LimitTracker<'a, T: Messenger> {
	messenger: &'a T,
	value: usize,
	max: usize,
}

impl <'a, T> LimitTracker<'a, T> 
where 
	T: Messenger,
{
	pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
		LimitTracker { 
			messenger,
			value: 0, 
			max,
		 }
	}

	pub fn set_value(&mut self, value: usize) {
		self.value = value;   

		let percentage_of_max = self.value as f64 / self.max as f64;

		if percentage_of_max >= 1.0 {
			self.messenger.send("Error: You are iver your quota!");
		} else if percentage_of_max >= 0.9 {
			self.messenger.send("Urgent Warning: You've used up over 90% of your quota!");
		} else if percentage_of_max >= 0.75 {
			self.messenger.send("Warning: You've used up over 75% of your quota!");
		}
	}
}
#[cfg(test)]
mod tests {
    use super::*;
	use std::cell::RefCell;

	struct MockMessenger {
		// We use here RefCell to demo the Inerior Mutability pattern.
		sent_messages: RefCell<Vec<String>>,
	}

	impl MockMessenger {
		fn new() -> MockMessenger {
			MockMessenger { 
				sent_messages: RefCell::new(vec![]),
			}
		}
	}

	impl Messenger for MockMessenger {
    fn send(&self, msg: &str) {
		// This is where we use the interior mutability.
		// We can't push to a regular vector since self is immutable.
		// We can't burrow a mutable self because it won't match the library's
		// method signature.
		// So we burrow a mutable reference of the vector inside this method
        self.sent_messages.borrow_mut().push(msg.to_string());
    }
}
    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
		let mock_messenger = MockMessenger::new();
		let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

		limit_tracker.set_value(80);

		assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
