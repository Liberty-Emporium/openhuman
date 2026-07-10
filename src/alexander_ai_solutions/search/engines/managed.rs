use crate::alexander_ai_solutions::config::Config;
use crate::alexander_ai_solutions::search::registry::SearchToolParams;
use crate::alexander_ai_solutions::tools::Tool;

pub(crate) fn build(root_config: &Config, params: SearchToolParams) -> Vec<Box<dyn Tool>> {
    tracing::debug!(
        requested = %root_config.search.requested_engine_str(),
        "[search] active engine = managed (backend-proxied web_search)"
    );

    vec![Box::new(
        crate::alexander_ai_solutions::search::WebSearchTool::new(
            crate::alexander_ai_solutions::integrations::build_client(root_config),
            params.max_results,
            params.timeout_secs,
        ),
    )]
}
