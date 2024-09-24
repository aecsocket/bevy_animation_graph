use crate::core::animation_graph::PinMap;
use crate::core::animation_node::{AnimationNode, AnimationNodeType, NodeLike};
use crate::core::errors::GraphError;
use crate::core::prelude::DataSpec;
use crate::prelude::{PassContext, SpecContext};
use crate::utils::unwrap::UnwrapVal;
use bevy::prelude::*;

#[derive(Reflect, Clone, Debug, Default)]
#[reflect(Default)]
pub struct SelectF32 {}

impl SelectF32 {
    pub const INPUT_BOOL: &'static str = "bool";
    pub const INPUT_FALSE: &'static str = "if false";
    pub const INPUT_TRUE: &'static str = "if true";
    pub const OUTPUT: &'static str = "out";

    pub fn new() -> Self {
        Self {}
    }

    pub fn wrapped(self, name: impl Into<String>) -> AnimationNode {
        AnimationNode::new_from_nodetype(name.into(), AnimationNodeType::SelectF32(self))
    }
}

impl NodeLike for SelectF32 {
    fn display_name(&self) -> String {
        "Select F32".into()
    }

    fn update(&self, mut ctx: PassContext) -> Result<(), GraphError> {
        let bool: bool = ctx.data_back(Self::INPUT_BOOL)?.val();
        let if_false: f32 = ctx.data_back(Self::INPUT_FALSE)?.val();
        let if_true: f32 = ctx.data_back(Self::INPUT_TRUE)?.val();

        let output = if bool { if_true } else { if_false };

        ctx.set_data_fwd(Self::OUTPUT, output);
        Ok(())
    }

    fn data_input_spec(&self, _ctx: SpecContext) -> PinMap<DataSpec> {
        [
            (Self::INPUT_BOOL.into(), DataSpec::Bool),
            (Self::INPUT_FALSE.into(), DataSpec::F32),
            (Self::INPUT_TRUE.into(), DataSpec::F32),
        ]
        .into()
    }

    fn data_output_spec(&self, _ctx: SpecContext) -> PinMap<DataSpec> {
        [(Self::OUTPUT.into(), DataSpec::F32)].into()
    }
}