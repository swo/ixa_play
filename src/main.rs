mod contacts;
mod incidence_report;
mod infection_manager;
mod people;

use ixa::prelude::*;
use ixa::run_with_args;

static I0: u64 = 1;
static GI: f64 = 10.0;
static MAX_TIME: f64 = 25.0;
static N_OFFSPRING: usize = 3;

fn main() {
    let result = run_with_args(|context: &mut Context, _args, _| {
        // Shut down the simulation after `max_time`
        context.add_plan(MAX_TIME, |context| {
            context.shutdown();
        });
        incidence_report::init(context, _args.output_dir.expect("no output dir"))
            .expect("Failed to init incidence report");
        infection_manager::init(context);
        contacts::init(context);
        people::init(context);
        Ok(())
    });

    match result {
        Ok(_) => {
            info!("Simulation finished executing");
        }
        Err(e) => {
            error!("Simulation exited with error: {}", e);
        }
    }
}
