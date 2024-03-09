#![allow(dead_code)]

use bevy::prelude::*;
use enumset::{EnumSet, EnumSetType};

/// EnumSet<Layer> is a set of layers
#[derive(EnumSetType, Debug, Hash)]
pub enum Layer {
    Garden,
    Parent,
    Child,
    Tool,
}

#[derive(Component, Debug)]
pub struct InLayers {
    pub layers: EnumSet<Layer>,
}

impl InLayers {
    pub fn new(layers: EnumSet<Layer>) -> Self {
        InLayers { layers }
    }

    pub fn new_single(layer: Layer) -> Self {
        InLayers {
            layers: EnumSet::only(layer),
        }
    }

    pub fn new_empty() -> Self {
        InLayers {
            layers: EnumSet::empty(),
        }
    }

    pub fn new_all() -> Self {
        InLayers {
            layers: EnumSet::all(),
        }
    }

    pub fn contains(&self, layer: Layer) -> bool {
        self.layers.contains(layer)
    }

    pub fn intersects(&self, other: &InLayers) -> bool {
        !self.layers.is_disjoint(other.layers)
    }

    pub fn intersects_layer_set(&self, other: EnumSet<Layer>) -> bool {
        !self.layers.is_disjoint(other)
    }
}