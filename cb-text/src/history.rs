struct Node<P: Clone> {
    parent_id: Option<usize>,
    branch_ids: Vec<usize>,
    payload: P,
}

pub struct HistoryTree<P: Clone> {
    nodes: Vec<Node<P>>,
    current_node_id: usize,
}

impl<P: Clone> HistoryTree<P> {
    pub fn new(payload: P) -> Self {
        HistoryTree {
            nodes: vec![Node {
                parent_id: None,
                branch_ids: Vec::new(),
                payload: payload,
            }],
            current_node_id: 0,
        }
    }

    fn current_node(&self) -> &Node<P> {
        &self.nodes[self.current_node_id]
    }

    fn current_node_mut(&mut self) -> &mut Node<P> {
        &mut self.nodes[self.current_node_id]
    }

    pub fn current_payload(&self) -> P {
        let current_node = self.current_node();

        current_node.payload.clone()
    }

    pub fn branch_payload(&self, branch_id: usize) -> Option<P> {
        if branch_id < self.nodes.len() {
            let node = &self.nodes[branch_id];
            Some(node.payload.clone())
        } else {
            None
        }
    }

    pub fn branch_ids(&self) -> Vec<usize> {
        self.current_node().branch_ids.clone()
    }

    pub fn push(&mut self, payload: P) {
        let next_node_id = self.nodes.len();
        self.current_node_mut().branch_ids.push(next_node_id);

        self.nodes.push(Node {
            parent_id: Some(self.current_node_id),
            branch_ids: Vec::new(),
            payload: payload,
        });

        self.current_node_id = self.nodes.len() - 1;
    }

    pub fn forward_branch(&mut self, branch_id: usize) -> Option<P> {
        let next_node_id = if self.current_node().branch_ids.contains(&branch_id) {
            Some(branch_id)
        } else {
            None
        };

        match next_node_id {
            Some(id) => {
                self.current_node_id = id;

                Some(self.current_node().payload.clone())
            }
            None => None,
        }
    }

    pub fn forward(&mut self) -> Option<P> {
        if let Some(latest_branch_id) = self.current_node().branch_ids.last() {
            self.current_node_id = *latest_branch_id;

            Some(self.current_node().payload.clone())
        } else {
            None
        }
    }

    pub fn back(&mut self) -> Option<P> {
        let current_node = &self.nodes[self.current_node_id];

        let payload = current_node.payload.clone();

        if let Some(parent_id) = current_node.parent_id {
            self.current_node_id = parent_id;

            Some(payload)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn walks_the_tree() {
        let mut history: HistoryTree<&str> = HistoryTree::new("@");

        assert_eq!(history.current_payload(), "@");

        // Start with linear history

        history.push("a");
        history.push("b");
        history.push("c");
        assert_eq!(history.current_payload(), "c");

        // Go back

        assert_eq!(history.back(), Some("c"));
        assert_eq!(history.current_payload(), "b");

        assert_eq!(history.back(), Some("b"));
        assert_eq!(history.current_payload(), "a");

        // Start branch

        history.push("B");
        assert_eq!(history.current_payload(), "B");

        history.push("C");
        assert_eq!(history.current_payload(), "C");

        // Go back to branch point

        assert_eq!(history.back(), Some("C"));
        assert_eq!(history.current_payload(), "B");

        assert_eq!(history.back(), Some("B"));
        assert_eq!(history.current_payload(), "a");

        // Check available branches

        let branch_ids = history.branch_ids();

        assert_eq!(branch_ids.len(), 2);

        assert_eq!(history.branch_payload(branch_ids[0]), Some("b"));
        assert_eq!(history.branch_payload(branch_ids[1]), Some("B"));

        // Go forward with default branch selecton (last branch)

        assert_eq!(history.forward(), Some("B"));
        assert_eq!(history.forward(), Some("C"));
        assert_eq!(history.forward(), None);

        // Go back to branch point

        history.back();
        history.back();

        assert_eq!(history.current_payload(), "a");

        // Go forward with specific branch

        let branch_ids = history.branch_ids();

        assert_eq!(branch_ids.len(), 2);

        assert_eq!(history.forward_branch(branch_ids[0]), Some("b"));
        assert_eq!(history.forward(), Some("c"));
        assert_eq!(history.forward(), None);
    }
}

