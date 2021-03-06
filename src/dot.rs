use std::io::Write;
use std::borrow::Cow;

use crate::graph::Graph;
use std::fmt::Display;

/*
   Bounds on types throw warnings
   type Nd<D: Display + Clone + Ord> = D;
   type Ed<D: Display + Clone + Ord> = (D, D);
   */
type Nd<D> = D;
type Ed<D> = (D, D);

pub(crate) struct Edges<D: Display + Clone>(pub(crate) Vec<Ed<D>>);

impl<'a, D: Display + Clone + Ord> dot::Labeller<'a, Nd<D>, Ed<D>> for Edges<D> {
    //TODO make it possible to rename Id
    fn graph_id(&'a self) -> dot::Id<'a> { dot::Id::new("example1").unwrap() }

    fn node_id(&'a self, n: &Nd<D>) -> dot::Id<'a> {
        dot::Id::new(format!("{}", *n)).unwrap()
    }
}

impl <'a, D: Display + Clone + Ord> dot::GraphWalk<'a, Nd<D>, Ed<D>> for Edges<D> {
    fn nodes(&self) -> dot::Nodes<'a, Nd<D>> {
        let &Edges(ref v) = self;
        let mut nodes = Vec::with_capacity(v.len());

        for (s, t) in v.iter() {
            nodes.push(s.clone()); nodes.push(t.clone());
        }

        nodes.sort();
        nodes.dedup();
        Cow::Owned(nodes)
    }

    fn edges(&'a self) -> dot::Edges<'a,Ed<D>> {
        let &Edges(ref edges) = self;
        Cow::Borrowed(&edges[..])
    }

    fn source(&self, e: &Ed<D>) -> Nd<D> { e.0.clone() }
    fn target(&self, e: &Ed<D>) -> Nd<D> { e.1.clone() }
}
