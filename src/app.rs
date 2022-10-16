use crate::browser;
use crate::image::Renderer;
use crate::plot::Plot;
use anyhow::Result;

pub struct App {}

// type SharedLoopClosure = Rc<RefCell<Option<LoopClosure>>>;

impl App {
    pub async fn start(plot: impl Plot + 'static) -> Result<()> {
        let plot = plot.initialize().await?;
        let renderer = Renderer {
            context: browser::context()?,
        };

        plot.draw(&renderer);

        // let f: SharedLoopClosure = Rc::new(RefCell::new(None));
        // let g = f.clone();
        // *g.borrow_mut() = Some(browser::create_raf_closure(move |perf: f64| {
        //     plot.draw(&renderer);
        // }));

        Ok(())
    }
}
