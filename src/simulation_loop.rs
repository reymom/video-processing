use crate::browser::{context, create_raf_closure, now, request_animation_frame, LoopClosure};
use crate::image::Renderer;
use anyhow::anyhow;
use anyhow::Result;
use async_trait::async_trait;
use std::cell::RefCell;
use std::rc::Rc;

#[async_trait(?Send)]
pub trait Simulation {
    async fn initialize(&self) -> Result<Box<dyn Simulation>>;
    fn update(&mut self);
    fn draw(&self, render: &Renderer);
}

pub struct SimulationLoop {
    last_frame: f64,
    accumulated_delta: f32,
}

type SharedLoopClosure = Rc<RefCell<Option<LoopClosure>>>;

const FRAME_SIZE: f32 = 1.0 / 60.0 * 1000.0;

impl SimulationLoop {
    pub async fn start(plot: impl Simulation + 'static) -> Result<()> {
        let mut plot = plot.initialize().await?;
        let mut simulation = SimulationLoop {
            last_frame: now()?,
            accumulated_delta: 0.0,
        };
        let renderer = Renderer {
            context: context()?,
        };

        let f: SharedLoopClosure = Rc::new(RefCell::new(None));
        let g = f.clone();
        *g.borrow_mut() = Some(create_raf_closure(move |perf: f64| {
            let frame_time = perf - simulation.last_frame;
            simulation.accumulated_delta += frame_time as f32;
            while simulation.accumulated_delta > FRAME_SIZE {
                simulation.accumulated_delta -= FRAME_SIZE;
            }
            simulation.last_frame = perf;
            plot.update();
            plot.draw(&renderer);
        }));

        request_animation_frame(
            g.borrow()
                .as_ref()
                .ok_or_else(|| anyhow!("Simulation: Loop is None"))?,
        )?;

        Ok(())
    }
}
