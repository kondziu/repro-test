use djanco::*;
use djanco::log::*;
use djanco::utils::*;
use clap::Clap;

use style_analyzer_query;

const PROJECT_NAME: &'static str = "style_analyzer_query";

pub fn main() {

    let options = CommandLineOptions::parse();
    let log = Log::new(options.verbosity);
    let dataset = options.dataset_path_as_str();
    let cache = options.cache_path_as_str();

    let repository = if let Some(repository) = options.repository.as_ref() {
        Some(create_project_archive(PROJECT_NAME, repository.as_str()))
    } else {
        None
    };

    macro_rules! execute_query {
        ($database:expr, $method:path) => {
            timed_query!($method[&$database, &log, &options.output_path]);
        }
    }

    macro_rules! prepare_database {
        ($savepoint:expr, $stores:expr) => {
            Djanco::from_spec(dataset, cache, $savepoint, $stores, log.clone())
                .expect("Error initializing Djanco!");
        }
    }

    let database = prepare_database!(1619827200 /* = May 2021*/, stores!(JavaScript));
    execute_query!(database, style_analyzer_query::all_projects);
    execute_query!(database, style_analyzer_query::project_locs);
    execute_query!(database, style_analyzer_query::random_projects_0);
    execute_query!(database, style_analyzer_query::random_projects_1);
    execute_query!(database, style_analyzer_query::random_projects_2);
    execute_query!(database, style_analyzer_query::random_projects_3);
    execute_query!(database, style_analyzer_query::random_projects_4);
    execute_query!(database, style_analyzer_query::random_projects_5);
    execute_query!(database, style_analyzer_query::random_projects_6);
    execute_query!(database, style_analyzer_query::random_projects_7);
    execute_query!(database, style_analyzer_query::random_projects_8);
    execute_query!(database, style_analyzer_query::random_projects_9);
    execute_query!(database, style_analyzer_query::random_projects_by_size_0);
    execute_query!(database, style_analyzer_query::random_projects_by_size_1);
    execute_query!(database, style_analyzer_query::random_projects_by_size_2);
    execute_query!(database, style_analyzer_query::random_projects_by_size_3);
    execute_query!(database, style_analyzer_query::random_projects_by_size_4);
    execute_query!(database, style_analyzer_query::random_projects_by_size_5);
    execute_query!(database, style_analyzer_query::random_projects_by_size_6);
    execute_query!(database, style_analyzer_query::random_projects_by_size_7);
    execute_query!(database, style_analyzer_query::random_projects_by_size_8);
    execute_query!(database, style_analyzer_query::random_projects_by_size_9);
    execute_query!(database, style_analyzer_query::top_starred);


    if options.repository.is_some() && !options.do_not_archive_results {
        add_results(PROJECT_NAME, &repository.unwrap(), &options.output_path, options.size_limit);
    }
}
