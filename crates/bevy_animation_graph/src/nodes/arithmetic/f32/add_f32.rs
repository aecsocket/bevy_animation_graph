use crate::core::animation_graph::PinMap;
use crate::core::animation_node::{NodeLike, ReflectNodeLike};
use crate::core::errors::GraphError;
use crate::core::prelude::DataSpec;
use crate::prelude::{PassContext, SpecContext};
use bevy::prelude::*;

#[derive(Reflect, Clone, Debug, Default)]
#[reflect(Default, NodeLike)]
pub struct AddF32 {}

impl AddF32 {
    pub const INPUT_1: &'static str = "F32 In 1";
    pub const INPUT_2: &'static str = "F32 In 2";
    pub const OUTPUT: &'static str = "F32 Out";

    pub fn new() -> Self {
        Self {}
    }
}

impl NodeLike for AddF32 {
    fn clone_value(&self) -> Box<dyn NodeLike> {
        Box::new(self.clone())
    }

    fn display_name(&self) -> String {
        "+ Add".into()
    }

    fn update(&self, mut ctx: PassContext) -> Result<(), GraphError> {
        let input_1 = ctx.data_back(Self::INPUT_1)?.unwrap_f32();
        let input_2 = ctx.data_back(Self::INPUT_2)?.unwrap_f32();
        ctx.set_data_fwd(Self::OUTPUT, input_1 + input_2);
        Ok(())
    }

    fn data_input_spec(&self, _ctx: SpecContext) -> PinMap<DataSpec> {
        [
            (Self::INPUT_1.into(), DataSpec::F32),
            (Self::INPUT_2.into(), DataSpec::F32),
        ]
        .into()
    }

    fn data_output_spec(&self, _ctx: SpecContext) -> PinMap<DataSpec> {
        [(Self::OUTPUT.into(), DataSpec::F32)].into()
    }
}
