use crate::algorithm::tree::RBTree;
use dioxus::prelude::*;

#[derive(Debug, Clone)]
pub struct Controls {
    pub ind: Signal<i32>,
    pub speed: Signal<i32>,
}

pub static SELECTED_TREE: GlobalSignal<String> = Signal::global(|| "Red-Black Tree".to_string());
pub static RED_BLACK_TREE: GlobalSignal<RBTree> = Signal::global(|| RBTree::new());
pub static RBTREE: GlobalSignal<RBTree> = Signal::global(|| RBTree::new());
pub static CONTROLS: GlobalSignal<Controls> = Signal::global(|| Controls {
    ind: Signal::new(0),
    speed: Signal::new(0),
});

pub static TREE_STATES: GlobalSignal<Vec<RBTree>> = Signal::global(Vec::new);
pub static SVG_VIEW_BOX: GlobalSignal<Vec<f32>> =
    Signal::global(|| vec![-50.0, -20.0, 300.0, 300.0]);
pub static STATUS: GlobalSignal<String> = Signal::global(|| "IDLE".to_string());
