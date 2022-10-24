use crate::canvas::{Image, Renderer};
use crate::plot_states::state_implementations::{End, PlotState, Ready, Simulating};
use futures::channel::mpsc::UnboundedReceiver;

pub enum PlotMachine {
    Ready(PlotState<Ready>),
    Simulating(PlotState<Simulating>),
    End(PlotState<End>),
}

impl PlotMachine {
    pub fn new(image: Image, button: UnboundedReceiver<()>) -> Self {
        PlotMachine::Ready(PlotState::new(image, button))
    }

    pub fn update(self) -> Self {
        match self {
            PlotMachine::Ready(state) => state.update().into(),
            PlotMachine::Simulating(state) => state.update().into(),
            PlotMachine::End(state) => state.update().into(),
        }
    }

    pub fn draw(&self, renderer: &Renderer) {
        match self {
            PlotMachine::Ready(state) => state.draw(renderer),
            PlotMachine::Simulating(state) => state.draw(renderer),
            PlotMachine::End(state) => state.draw(renderer),
        }
    }
}
