use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::core::{animation_graph::PinMap, edge_data::DataValue};

use super::{Extra, GlobalTransition, State, StateId, StateMachine, Transition, TransitionId};

pub type StateIdSerial = StateId;
pub type TransitionIdSerial = TransitionId;

#[derive(Serialize, Deserialize, Clone)]
pub struct StateSerial {
    pub id: StateIdSerial,
    pub graph: String,
    pub global_transition: Option<GlobalTransitionSerial>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GlobalTransitionSerial {
    pub duration: f32,
    pub graph: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TransitionSerial {
    pub id: TransitionIdSerial,
    pub source: StateIdSerial,
    pub target: StateIdSerial,
    pub duration: f32,
    pub graph: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct StateMachineSerial {
    pub states: Vec<StateSerial>,
    pub transitions: Vec<TransitionSerial>,
    pub start_state: String,
    #[serde(default)]
    pub input_data: PinMap<DataValue>,
    #[serde(default)]
    pub extra: Extra,
}

#[derive(Debug, Clone, Error)]
#[error("graph path missing")]
pub struct GraphPathMissing;

impl TryFrom<&Transition> for TransitionSerial {
    type Error = GraphPathMissing;

    fn try_from(value: &Transition) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id.clone(),
            source: value.source.clone(),
            target: value.target.clone(),
            duration: value.duration,
            graph: value.graph.path().ok_or(GraphPathMissing)?.to_string(),
        })
    }
}

impl TryFrom<&State> for StateSerial {
    type Error = StateMissing;

    fn try_from(value: &State) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id.clone(),
            graph: value
                .graph
                .path()
                .ok_or(StateMissing::GraphPath)?
                .to_string(),
            global_transition: value
                .global_transition
                .as_ref()
                .map(|transition| {
                    GlobalTransitionSerial::try_from(transition)
                        .map_err(|_| StateMissing::GlobalTransitionGraphPath)
                })
                .transpose()?,
        })
    }
}

#[derive(Debug, Clone, Error)]
pub enum StateMissing {
    #[error("graph path missing")]
    GraphPath,
    #[error("global transition graph path missing")]
    GlobalTransitionGraphPath,
}

impl TryFrom<&GlobalTransition> for GlobalTransitionSerial {
    type Error = GraphPathMissing;

    fn try_from(value: &GlobalTransition) -> Result<Self, Self::Error> {
        Ok(Self {
            duration: value.duration,
            graph: value.graph.path().ok_or(GraphPathMissing)?.to_string(),
        })
    }
}

impl TryFrom<&StateMachine> for StateMachineSerial {
    type Error = StateMachineMissing;

    fn try_from(value: &StateMachine) -> Result<Self, Self::Error> {
        Ok(Self {
            states: value
                .states
                .values()
                .map(|state| StateSerial::try_from(state).map_err(StateMachineMissing::State))
                .collect::<Result<_, _>>()?,
            transitions: value
                .transitions
                .values()
                .map(|transition| {
                    TransitionSerial::try_from(transition).map_err(StateMachineMissing::Transition)
                })
                .collect::<Result<_, _>>()?,
            start_state: value.start_state.clone(),
            extra: value.extra.clone(),
            input_data: value.input_data.clone(),
        })
    }
}

#[derive(Debug, Clone, Error)]
pub enum StateMachineMissing {
    #[error("state missing")]
    State(#[source] StateMissing),
    #[error("transition missing")]
    Transition(#[source] GraphPathMissing),
}
