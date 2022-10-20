pub mod state_implementations {
    use crate::browser;
    use crate::button;
    use crate::constants::*;
    use crate::image::{Image, Renderer};
    use crate::plot_machine::PlotMachine;
    use futures::channel::mpsc::UnboundedReceiver;

    pub struct PlotState<T> {
        _state: T,
        plot: Image,
    }

    impl<T> PlotState<T> {
        pub fn draw(&self, renderer: &Renderer) {
            self.plot.draw(renderer);
        }
    }

    pub struct Ready {
        start_event: UnboundedReceiver<()>,
    }

    impl From<PlotState<Ready>> for PlotMachine {
        fn from(state: PlotState<Ready>) -> Self {
            PlotMachine::Ready(state)
        }
    }

    pub struct Simulating {
        pause_event: UnboundedReceiver<()>,
        finish_event: UnboundedReceiver<()>,
    }

    impl From<PlotState<Simulating>> for PlotMachine {
        fn from(state: PlotState<Simulating>) -> Self {
            PlotMachine::Simulating(state)
        }
    }

    pub struct End {
        refresh_event: UnboundedReceiver<()>,
        save_event: UnboundedReceiver<()>,
    }

    impl From<PlotState<End>> for PlotMachine {
        fn from(state: PlotState<End>) -> Self {
            PlotMachine::End(state)
        }
    }

    // ---------------------
    // - implementation of -
    // -      Ready        -
    // ---------------------

    impl Ready {
        fn run_simulation_pressed(&mut self) -> bool {
            matches!(self.start_event.try_next(), Ok(Some(())))
        }
    }

    pub enum ReadyStateTransition {
        Simulate(PlotState<Simulating>),
        Same(PlotState<Ready>),
    }

    impl From<ReadyStateTransition> for PlotMachine {
        fn from(state: ReadyStateTransition) -> Self {
            match state {
                ReadyStateTransition::Simulate(simulating) => simulating.into(),
                ReadyStateTransition::Same(ready) => ready.into(),
            }
        }
    }

    impl PlotState<Ready> {
        pub fn new(image: Image, button: UnboundedReceiver<()>) -> PlotState<Ready> {
            PlotState {
                _state: Ready {
                    start_event: button,
                },
                plot: image,
            }
        }

        pub fn update(mut self) -> ReadyStateTransition {
            if self._state.run_simulation_pressed() {
                ReadyStateTransition::Simulate(self.start_simulation())
            } else {
                ReadyStateTransition::Same(self)
            }
        }

        fn start_simulation(self) -> PlotState<Simulating> {
            if let Err(err) = browser::hide_ui() {
                error!("Error hiding the browser {:#?}", err);
            }
            let pause_event = browser::draw_ui(pause_simulation_button)
                .and_then(|_unit| browser::find_html_element_by_id(pause_simulation_id))
                .map(button::add_click_handler)
                .unwrap();
            let finish_event = browser::draw_ui(finish_simulation_button)
                .and_then(|_unit| browser::find_html_element_by_id(finish_simulation_id))
                .map(button::add_click_handler)
                .unwrap();
            PlotState {
                _state: Simulating {
                    pause_event,
                    finish_event,
                },
                plot: self.plot,
            }
        }
    }

    // ---------------------
    // - implementation of -
    // -    Simulating     -
    // ---------------------

    impl Simulating {
        fn pause_simulation_pressed(&mut self) -> bool {
            matches!(self.pause_event.try_next(), Ok(Some(())))
        }

        fn finish_simulation_pressed(&mut self) -> bool {
            matches!(self.finish_event.try_next(), Ok(Some(())))
        }
    }

    pub enum SimulatingStateTransition {
        Pause(PlotState<Ready>),
        Finish(PlotState<End>),
        Simulate(PlotState<Simulating>),
    }

    impl From<SimulatingStateTransition> for PlotMachine {
        fn from(state: SimulatingStateTransition) -> Self {
            match state {
                SimulatingStateTransition::Pause(ready) => ready.into(),
                SimulatingStateTransition::Finish(end) => end.into(),
                SimulatingStateTransition::Simulate(simulating) => simulating.into(),
            }
        }
    }

    impl PlotState<Simulating> {
        pub fn update(mut self) -> SimulatingStateTransition {
            if self._state.pause_simulation_pressed() {
                SimulatingStateTransition::Pause(self.pause_simulation())
            } else if self._state.finish_simulation_pressed() {
                SimulatingStateTransition::Finish(self.finish_simulation())
            } else {
                SimulatingStateTransition::Simulate(self.run_simulation_step())
            }
        }

        fn pause_simulation(self) -> PlotState<Ready> {
            if let Err(err) = browser::hide_ui() {
                error!("Error hiding the browser {:#?}", err);
            }
            let start_event = browser::draw_ui(run_simulation_button)
                .and_then(|_unit| browser::find_html_element_by_id(run_simulation_id))
                .map(button::add_click_handler)
                .unwrap();
            PlotState {
                _state: Ready { start_event },
                plot: self.plot,
            }
        }

        fn finish_simulation(self) -> PlotState<End> {
            if let Err(err) = browser::hide_ui() {
                error!("Error hiding the browser {:#?}", err);
            }
            let refresh_event = browser::draw_ui(run_simulation_button)
                .and_then(|_unit| browser::find_html_element_by_id(run_simulation_id))
                .map(button::add_click_handler)
                .unwrap();
            let save_event = browser::draw_ui(save_image_button)
                .and_then(|_unit| browser::find_html_element_by_id(save_image_id))
                .map(button::add_click_handler)
                .unwrap();
            PlotState {
                _state: End {
                    refresh_event,
                    save_event,
                },
                plot: self.plot,
            }
        }

        fn run_simulation_step(mut self) -> PlotState<Simulating> {
            self.plot.run_simulation_step();
            self
        }
    }

    // ---------------------
    // - implementation of -
    // -       End         -
    // ---------------------

    impl End {
        fn refresh_image_pressed(&mut self) -> bool {
            matches!(self.refresh_event.try_next(), Ok(Some(())))
        }

        fn save_image_pressed(&mut self) -> bool {
            matches!(self.save_event.try_next(), Ok(Some(())))
        }
    }

    pub enum EndStateTransition {
        Refresh(PlotState<Ready>),
        Save(PlotState<End>),
        Continue(PlotState<End>),
    }

    impl From<EndStateTransition> for PlotMachine {
        fn from(state: EndStateTransition) -> Self {
            match state {
                EndStateTransition::Refresh(ready) => ready.into(),
                EndStateTransition::Save(end) => end.into(),
                EndStateTransition::Continue(end) => end.into(),
            }
        }
    }

    impl PlotState<End> {
        pub fn update(mut self) -> EndStateTransition {
            if self._state.refresh_image_pressed() {
                EndStateTransition::Refresh(self.refresh_image())
            } else if self._state.save_image_pressed() {
                EndStateTransition::Save(self.save_image())
            } else {
                EndStateTransition::Continue(self)
            }
        }

        fn refresh_image(mut self) -> PlotState<Ready> {
            if let Err(err) = browser::hide_ui() {
                error!("Error hiding the browser {:#?}", err);
            }
            let start_event = browser::draw_ui(run_simulation_button)
                .and_then(|_unit| browser::find_html_element_by_id(run_simulation_id))
                .map(button::add_click_handler)
                .unwrap();
            self.plot.refresh();
            PlotState {
                _state: Ready { start_event },
                plot: self.plot,
            }
        }

        fn save_image(self) -> PlotState<End> {
            /*
            todo: implement save image
            */
            self
        }
    }
}
