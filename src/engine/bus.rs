pub trait Node {
    type Event;
    fn receive(&mut self, event: &Self::Event) -> Option<Vec<Self::Event>>;
}

pub struct Bus<T> {
    nodes: Vec<Box<dyn Node<Event = T>>>
}

impl<A> Bus<A> {
    pub fn post(&mut self, event: A) {
        let mut queue: Vec<A> = vec!();
        for node in self.nodes.iter_mut() {
            if let Some(events) = node.receive(&event) {
                for event in events {
                    queue.push(event);
                }
            }
        }
        for event in queue {
            self.post(event);
        }
    }

    pub fn connect<B>(&mut self, node: B)
    where
        B: Node<Event = A> + 'static {
        self.nodes.push(Box::new(node));
    }
}

impl<T> Default for Bus<T> {
    fn default() -> Self {
        let nodes: Vec<_> = vec!();
        Self {
            nodes
        }
    }
}