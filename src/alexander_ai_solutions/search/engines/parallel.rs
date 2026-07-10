use std::sync::Arc;

use crate::alexander_ai_solutions::config::Config;
use crate::alexander_ai_solutions::search::registry::SearchToolParams;
use crate::alexander_ai_solutions::tools::Tool;

pub(crate) fn build(root_config: &Config, params: SearchToolParams) -> Vec<Box<dyn Tool>> {
    tracing::debug!("[search] active engine = parallel (BYO direct API)");

    let client = crate::alexander_ai_solutions::integrations::build_client(root_config);
    let Some(client) = client else {
        tracing::warn!(
            "[search] engine=parallel but no backend client — falling back to managed surface"
        );
        return vec![Box::new(
            crate::alexander_ai_solutions::search::WebSearchTool::new(
                None,
                params.max_results,
                params.timeout_secs,
            ),
        )];
    };

    vec![
        Box::new(
            crate::alexander_ai_solutions::search::tools::ParallelSearchTool::new(Arc::clone(
                &client,
            )),
        ),
        Box::new(
            crate::alexander_ai_solutions::search::tools::ParallelExtractTool::new(Arc::clone(
                &client,
            )),
        ),
        Box::new(
            crate::alexander_ai_solutions::search::tools::ParallelChatTool::new(Arc::clone(
                &client,
            )),
        ),
        Box::new(
            crate::alexander_ai_solutions::search::tools::ParallelResearchTool::new(Arc::clone(
                &client,
            )),
        ),
        Box::new(
            crate::alexander_ai_solutions::search::tools::ParallelEnrichTool::new(Arc::clone(
                &client,
            )),
        ),
        Box::new(
            crate::alexander_ai_solutions::search::tools::ParallelDatasetTool::new(Arc::clone(
                &client,
            )),
        ),
        Box::new(crate::alexander_ai_solutions::search::WebSearchTool::new(
            Some(Arc::clone(&client)),
            params.max_results,
            params.timeout_secs,
        )),
    ]
}
