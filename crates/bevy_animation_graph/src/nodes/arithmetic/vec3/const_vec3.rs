use crate::core::animation_graph::PinMap;
use crate::core::animation_node::{NodeLike, ReflectNodeLike};
use crate::core::errors::GraphError;
use crate::core::prelude::DataSpec;
use crate::prelude::{PassContext, SpecContext};
use bevy::prelude::*;

#[derive(Reflect, Clone, Debug, Default)]
#[reflect(Default, NodeLike)]
#[type_path = "bevy_animation_graph::node::vec3"]
#[type_name = "Const"]
pub struct ConstVec3Node(pub Vec3);

impl ConstVec3Node {
    pub const OUTPUT: &'static str = "out";

    pub fn new(constant: Vec3) -> Self {
        Self(constant)
    }
}

impl NodeLike for ConstVec3Node {
    fn display_name(&self) -> String {
        "Vec3".into()
    }

    fn update(&self, mut ctx: PassContext) -> Result<(), GraphError> {
        ctx.set_data_fwd(Self::OUTPUT, self.0);
        Ok(())
    }

    fn data_output_spec(&self, _ctx: SpecContext) -> PinMap<DataSpec> {
        [(Self::OUTPUT.into(), DataSpec::Vec3)].into()
    }
}
