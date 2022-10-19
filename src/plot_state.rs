use crate::browser;
use crate::button;
use crate::image::{Image, Renderer};
use futures::channel::mpsc::{unbounded, UnboundedReceiver};

pub enum PlotMachine {
    Ready(PlotState<Ready>),
    Simulating(PlotState<Simulating>),
    End(PlotState<End>),
}

pub struct PlotState<T> {
    _state: T,
    plot: Image,
}

pub struct Ready {
    start_event: UnboundedReceiver<()>,
}

struct Simulating {
    finish_event: UnboundedReceiver<()>,
}

struct End;

impl PlotMachine {
    pub fn new(image: Image, button: UnboundedReceiver<()>) -> Self {
        PlotMachine::Ready(PlotState {
            _state: Ready {
                start_event: button,
            },
            plot: image,
        })
    }

    pub fn update(&self) -> Self {
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

impl Ready {
    fn run_simulation_pressed(&mut self) -> bool {
        matches!(self.start_event.try_next(), Ok(Some(())))
    }
}

enum ReadyStateEnds {
    Simulate(PlotState<Simulating>),
    Same(PlotState<Ready>),
}

impl PlotState<Ready> {
    fn update(self) -> ReadyStateEnds {
        if self._state.run_simulation_pressed() {
            ReadyStateEnds::Simulate(self.start_simulation())
        } else {
            ReadyStateEnds::Same(self)
        }
    }

    fn start_simulation(self) -> PlotState<Simulating> {
        if let Err(err) = browser::hide_ui() {
            error!("Error hiding the browser {:#?}", err);
        }
        let finish_event = browser::draw_ui("<button id='end_simulation'>End simulation</button>")
            .and_then(|_unit| browser::find_html_element_by_id("end_simulation"))
            .map(button::add_click_handler)
            .unwrap();
        PlotState {
            _state: Simulating { finish_event },
            plot: self.plot,
        }
    }
}
