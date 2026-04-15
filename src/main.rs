mod incidence_report;
mod infection;
mod population;
mod vaccine;

use ixa::prelude::*;
use ixa::run_with_args;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParametersValues {
    pub i0: usize,
    pub gi: f64,
    pub max_time: f64,
    pub n_offspring: usize,
    pub p_vax: f64,
}

define_global_property!(Parameters, ParametersValues);

fn main() {
    run_with_args(|context: &mut Context, _args, _| {
        init(context, _args.output_dir.expect("no output dir"));
        Ok(())
    })
    .unwrap();
}

fn init(context: &mut Context, output_dir: std::path::PathBuf) {
    // note that the config is loaded automatically

    let parameters = context
        .get_global_property_value(Parameters)
        .unwrap()
        .clone();

    // Shut down the simulation after `max_time`
    context.add_plan(parameters.max_time, |context| {
        context.shutdown();
    });

    // initialize the modules
    incidence_report::init(context, output_dir).expect("Failed to init incidence report");
    infection::init(context, parameters.gi);
    population::init(context, parameters.i0);
    vaccine::init(context, parameters.p_vax);
}
