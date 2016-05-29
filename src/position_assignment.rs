use petgraph::{Graph, Directed, EdgeDirection};
use petgraph::graph::NodeIndex;
use super::graph::{Node, Edge};
use super::median::median;

fn segment(
    graph: &Graph<Node, Edge, Directed>,
    h1: &Vec<NodeIndex>
) -> (Vec<(NodeIndex, NodeIndex)>, Vec<(NodeIndex, NodeIndex)>){
    let mut inner = vec![];
    let mut outer = vec![];
    for u in h1 {
        for v in graph.neighbors(*u) {
            if graph[*u].dummy && graph[v].dummy {
                inner.push((*u, v));
            } else {
                outer.push((*u, v));
            }
        }
    }
    (inner, outer)
}

pub fn mark_conflicts(
    graph: &mut Graph<Node, Edge, Directed>,
    layers: &Vec<Vec<NodeIndex>>
) {
    for i in 1..(layers.len() - 1) {
        let h1 = layers.get(i).unwrap();
        let (inner, outer) = segment(graph, &h1);
        for (u1, v1) in inner {
            for &(u2, v2) in &outer {
                let ou1 = graph[u1].order;
                let ou2 = graph[u2].order;
                let ov1 = graph[v1].order;
                let ov2 = graph[v2].order;
                if (ou1 < ou2 && ov1 > ov2) || (ou1 > ou2 && ov1 < ov2) {
                    let index = graph.find_edge(u2, v2).unwrap();
                    graph[index].conflict = true;
                }
            }
        }
    }
}

pub fn vertical_alignment(
    graph: &mut Graph<Node, Edge, Directed>,
    layers: &Vec<Vec<NodeIndex>>
) {
    for u in graph.node_indices() {
        graph[u].root = Some(u);
        graph[u].align = Some(u);
    }
    for layer in layers.iter().skip(1) {
        let mut r = usize::min_value();
        for v in layer {
            match median(graph, *v, EdgeDirection::Incoming) {
                Some((left, right)) => {
                    let medians = if left == right {
                        vec![left]
                    } else {
                        vec![left, right]
                    };
                    for u in medians {
                        let edge = graph.find_edge(u, *v).unwrap();
                        if !graph[edge].conflict {
                            if r < graph[u].order {
                                graph[*v].align = graph[u].root;
                                graph[*v].root = graph[u].root;
                                graph[u].align = Some(*v);
                                r = graph[u].order;
                                break;
                            }
                        }
                    }
                },
                None => {},
            };
        }
    }
}

pub fn brandes(
    graph: &mut Graph<Node, Edge, Directed>,
    layers: &Vec<Vec<NodeIndex>>
) {
    mark_conflicts(graph, layers);
}

#[cfg(test)]
mod tests {
    use petgraph::Graph;
    use super::*;
    use super::super::graph::{Node, Edge};

    #[test]
    fn test_brandes() {
        let mut graph = Graph::new();
        let a1 = graph.add_node(Node::new());
        let a2 = graph.add_node(Node::new());
        let b1 = graph.add_node(Node::new());
        let b2 = graph.add_node(Node::new());
        let b3 = graph.add_node(Node::new());
        let b4 = graph.add_node(Node::new());
        let b5 = graph.add_node(Node::new());
        let b6 = graph.add_node(Node::new());
        let b7 = graph.add_node(Node::new());
        let b8 = graph.add_node(Node::new());
        let c1 = graph.add_node(Node::new());
        let c2 = graph.add_node(Node::new());
        let c3 = graph.add_node(Node::new());
        let c4 = graph.add_node(Node::new());
        let c5 = graph.add_node(Node::new());
        let c6 = graph.add_node(Node::new());
        let d1 = graph.add_node(Node::new());
        let d2 = graph.add_node(Node::new());
        let d3 = graph.add_node(Node::new());
        let d4 = graph.add_node(Node::new());
        let d5 = graph.add_node(Node::new());
        let d6 = graph.add_node(Node::new());
        let d7 = graph.add_node(Node::new());
        let e1 = graph.add_node(Node::new());
        let e2 = graph.add_node(Node::new());
        let e3 = graph.add_node(Node::new());
        graph.add_edge(a1, b1, Edge::new());
        graph.add_edge(a1, b6, Edge::new());
        graph.add_edge(a1, b8, Edge::new());
        graph.add_edge(a2, b3, Edge::new());
        graph.add_edge(a2, b5, Edge::new());
        graph.add_edge(b2, c2, Edge::new());
        graph.add_edge(b3, c2, Edge::new());
        graph.add_edge(b4, c2, Edge::new());
        graph.add_edge(b5, c3, Edge::new());
        graph.add_edge(b6, c4, Edge::new());
        let b7c2 = graph.add_edge(b7, c2, Edge::new());
        graph.add_edge(b7, c6, Edge::new());
        let b8c2 = graph.add_edge(b8, c2, Edge::new());
        graph.add_edge(b8, c5, Edge::new());
        graph.add_edge(c1, d1, Edge::new());
        graph.add_edge(c1, d2, Edge::new());
        let c1d6 = graph.add_edge(c1, d6, Edge::new());
        graph.add_edge(c3, d4, Edge::new());
        graph.add_edge(c4, d5, Edge::new());
        graph.add_edge(c5, d6, Edge::new());
        let c6d3 = graph.add_edge(c6, d3, Edge::new());
        graph.add_edge(c6, d7, Edge::new());
        graph.add_edge(d1, e1, Edge::new());
        graph.add_edge(d1, e2, Edge::new());
        graph.add_edge(d2, e2, Edge::new());
        graph.add_edge(d3, e1, Edge::new());
        graph.add_edge(d4, e3, Edge::new());
        graph.add_edge(d5, e3, Edge::new());
        graph.add_edge(d6, e3, Edge::new());
        graph.add_edge(d7, e3, Edge::new());
        graph[a1].order = 0;
        graph[a2].order = 1;
        graph[b1].order = 0;
        graph[b2].order = 1;
        graph[b3].order = 2;
        graph[b4].order = 3;
        graph[b5].order = 4;
        graph[b6].order = 5;
        graph[b7].order = 6;
        graph[b8].order = 7;
        graph[c1].order = 0;
        graph[c2].order = 1;
        graph[c3].order = 2;
        graph[c4].order = 3;
        graph[c5].order = 4;
        graph[c6].order = 5;
        graph[d1].order = 0;
        graph[d2].order = 1;
        graph[d3].order = 2;
        graph[d4].order = 3;
        graph[d5].order = 4;
        graph[d6].order = 5;
        graph[d7].order = 6;
        graph[e1].order = 0;
        graph[e2].order = 1;
        graph[e3].order = 2;
        graph[b3].dummy = true;
        graph[b5].dummy = true;
        graph[b6].dummy = true;
        graph[c3].dummy = true;
        graph[c4].dummy = true;
        graph[c5].dummy = true;
        graph[d3].dummy = true;
        graph[d4].dummy = true;
        graph[d5].dummy = true;
        graph[d7].dummy = true;
        let layers = vec![
            vec![a1, a2],
            vec![b1, b2, b3, b4, b5, b6, b7, b8],
            vec![c1, c2, c3, c4, c5, c6],
            vec![d1, d2, d3, d4, d5, d6, d7],
            vec![e1, e2, e3],
        ];
        mark_conflicts(&mut graph, &layers);
        assert!(graph[b7c2].conflict);
        assert!(graph[b8c2].conflict);
        assert!(graph[c1d6].conflict);
        assert!(graph[c6d3].conflict);
    }

    #[test]
    fn test_mark_conflicts() {
        let mut graph = Graph::new();
        let a = graph.add_node(Node::new());
        let b = graph.add_node(Node::new());
        let c = graph.add_node(Node::new());
        graph.add_edge(a, b, Edge::new());
        graph.add_edge(b, c, Edge::new());
        graph.add_edge(c, a, Edge::new());
        let layers = vec![
            vec![a],
            vec![b],
            vec![c],
        ];
        brandes(&mut graph, &layers);
    }

    #[test]
    fn test_vertical_alignment() {
        let mut graph = Graph::new();
        let a1 = graph.add_node(Node::new());
        let a2 = graph.add_node(Node::new());
        let b1 = graph.add_node(Node::new());
        let b2 = graph.add_node(Node::new());
        let b3 = graph.add_node(Node::new());
        let b4 = graph.add_node(Node::new());
        let b5 = graph.add_node(Node::new());
        let b6 = graph.add_node(Node::new());
        let b7 = graph.add_node(Node::new());
        let b8 = graph.add_node(Node::new());
        let c1 = graph.add_node(Node::new());
        let c2 = graph.add_node(Node::new());
        let c3 = graph.add_node(Node::new());
        let c4 = graph.add_node(Node::new());
        let c5 = graph.add_node(Node::new());
        let c6 = graph.add_node(Node::new());
        let d1 = graph.add_node(Node::new());
        let d2 = graph.add_node(Node::new());
        let d3 = graph.add_node(Node::new());
        let d4 = graph.add_node(Node::new());
        let d5 = graph.add_node(Node::new());
        let d6 = graph.add_node(Node::new());
        let d7 = graph.add_node(Node::new());
        let e1 = graph.add_node(Node::new());
        let e2 = graph.add_node(Node::new());
        let e3 = graph.add_node(Node::new());
        graph.add_edge(a1, b1, Edge::new());
        graph.add_edge(a1, b6, Edge::new());
        graph.add_edge(a1, b8, Edge::new());
        graph.add_edge(a2, b3, Edge::new());
        graph.add_edge(a2, b5, Edge::new());
        graph.add_edge(b2, c2, Edge::new());
        graph.add_edge(b3, c2, Edge::new());
        graph.add_edge(b4, c2, Edge::new());
        graph.add_edge(b5, c3, Edge::new());
        graph.add_edge(b6, c4, Edge::new());
        let b7c2 = graph.add_edge(b7, c2, Edge::new());
        graph.add_edge(b7, c6, Edge::new());
        let b8c2 = graph.add_edge(b8, c2, Edge::new());
        graph.add_edge(b8, c5, Edge::new());
        graph.add_edge(c1, d1, Edge::new());
        graph.add_edge(c1, d2, Edge::new());
        let c1d6 = graph.add_edge(c1, d6, Edge::new());
        graph.add_edge(c3, d4, Edge::new());
        graph.add_edge(c4, d5, Edge::new());
        graph.add_edge(c5, d6, Edge::new());
        let c6d3 = graph.add_edge(c6, d3, Edge::new());
        graph.add_edge(c6, d7, Edge::new());
        graph.add_edge(d1, e1, Edge::new());
        graph.add_edge(d1, e2, Edge::new());
        graph.add_edge(d2, e2, Edge::new());
        graph.add_edge(d3, e1, Edge::new());
        graph.add_edge(d4, e3, Edge::new());
        graph.add_edge(d5, e3, Edge::new());
        graph.add_edge(d6, e3, Edge::new());
        graph.add_edge(d7, e3, Edge::new());
        graph[a1].order = 1;
        graph[a2].order = 2;
        graph[b1].order = 1;
        graph[b2].order = 2;
        graph[b3].order = 3;
        graph[b4].order = 4;
        graph[b5].order = 5;
        graph[b6].order = 6;
        graph[b7].order = 7;
        graph[b8].order = 8;
        graph[c1].order = 1;
        graph[c2].order = 2;
        graph[c3].order = 3;
        graph[c4].order = 4;
        graph[c5].order = 5;
        graph[c6].order = 6;
        graph[d1].order = 1;
        graph[d2].order = 2;
        graph[d3].order = 3;
        graph[d4].order = 4;
        graph[d5].order = 5;
        graph[d6].order = 6;
        graph[d7].order = 7;
        graph[e1].order = 1;
        graph[e2].order = 2;
        graph[e3].order = 3;
        graph[b3].dummy = true;
        graph[b5].dummy = true;
        graph[b6].dummy = true;
        graph[c3].dummy = true;
        graph[c4].dummy = true;
        graph[c5].dummy = true;
        graph[d3].dummy = true;
        graph[d4].dummy = true;
        graph[d5].dummy = true;
        graph[d7].dummy = true;
        graph[b7c2].conflict = true;
        graph[b8c2].conflict = true;
        graph[c1d6].conflict = true;
        graph[c6d3].conflict = true;
        let layers = vec![
            vec![a1, a2],
            vec![b1, b2, b3, b4, b5, b6, b7, b8],
            vec![c1, c2, c3, c4, c5, c6],
            vec![d1, d2, d3, d4, d5, d6, d7],
            vec![e1, e2, e3],
        ];
        vertical_alignment(&mut graph, &layers);
        assert_eq!(graph[a1].root.unwrap(), a1);
        assert_eq!(graph[a1].align.unwrap(), b1);
        assert_eq!(graph[a2].root.unwrap(), a2);
        assert_eq!(graph[a2].align.unwrap(), b3);
        assert_eq!(graph[b1].root.unwrap(), a1);
        assert_eq!(graph[b1].align.unwrap(), a1);
        assert_eq!(graph[b2].root.unwrap(), b2);
        assert_eq!(graph[b2].align.unwrap(), b2);
        assert_eq!(graph[b3].root.unwrap(), a2);
        assert_eq!(graph[b3].align.unwrap(), a2);
        assert_eq!(graph[b4].root.unwrap(), b4);
        assert_eq!(graph[b4].align.unwrap(), c2);
        assert_eq!(graph[b5].root.unwrap(), b5);
        assert_eq!(graph[b5].align.unwrap(), c3);
        assert_eq!(graph[b6].root.unwrap(), b6);
        assert_eq!(graph[b6].align.unwrap(), c4);
        assert_eq!(graph[b7].root.unwrap(), b7);
        assert_eq!(graph[b7].align.unwrap(), b7);
        assert_eq!(graph[b8].root.unwrap(), b8);
        assert_eq!(graph[b8].align.unwrap(), c5);
        assert_eq!(graph[c1].root.unwrap(), c1);
        assert_eq!(graph[c1].align.unwrap(), d1);
        assert_eq!(graph[c2].root.unwrap(), b4);
        assert_eq!(graph[c2].align.unwrap(), b4);
        assert_eq!(graph[c3].root.unwrap(), b5);
        assert_eq!(graph[c3].align.unwrap(), d4);
        assert_eq!(graph[c4].root.unwrap(), b6);
        assert_eq!(graph[c4].align.unwrap(), d5);
        assert_eq!(graph[c5].root.unwrap(), b8);
        assert_eq!(graph[c5].align.unwrap(), d6);
        assert_eq!(graph[c6].root.unwrap(), c6);
        assert_eq!(graph[c6].align.unwrap(), d7);
        assert_eq!(graph[d1].root.unwrap(), c1);
        assert_eq!(graph[d1].align.unwrap(), e1);
        assert_eq!(graph[d2].root.unwrap(), d2);
        assert_eq!(graph[d2].align.unwrap(), e2);
        assert_eq!(graph[d3].root.unwrap(), d3);
        assert_eq!(graph[d3].align.unwrap(), d3);
        assert_eq!(graph[d4].root.unwrap(), b5);
        assert_eq!(graph[d4].align.unwrap(), b5);
        assert_eq!(graph[d5].root.unwrap(), b6);
        assert_eq!(graph[d5].align.unwrap(), e3);
        assert_eq!(graph[d6].root.unwrap(), b8);
        assert_eq!(graph[d6].align.unwrap(), b8);
        assert_eq!(graph[d7].root.unwrap(), c6);
        assert_eq!(graph[d7].align.unwrap(), c6);
        assert_eq!(graph[e1].root.unwrap(), c1);
        assert_eq!(graph[e1].align.unwrap(), c1);
        assert_eq!(graph[e2].root.unwrap(), d2);
        assert_eq!(graph[e2].align.unwrap(), d2);
        assert_eq!(graph[e3].root.unwrap(), b6);
        assert_eq!(graph[e3].align.unwrap(), b6);
    }
}
