use super::tools::{
    DeployPumpFunToken, GetCurrentTime, GetQuote, GetSolBalance,
    GetSplTokenBalance, Swap,
};
use crate::agents::research::ViewImage;
use crate::common::{
    claude_agent_builder, gemini_agent_builder, PREAMBLE_COMMON,
};
use crate::data::{
    AnalyzePageContent, FetchPriceActionAnalysis, FetchTokenMetadata,
    FetchTokenPrice, FetchTopTokens, FetchXPost, ResearchXProfile,
    SearchTweets, SearchWeb,
};
use crate::dexscreener::tools::SearchOnDexScreener;
use crate::faster100x::AnalyzeHolderDistribution;
use crate::lunarcrush::AnalyzeSentiment;
use crate::solana::advanced_orders::CreateAdvancedOrder;
use crate::solana::tools::AnalyzeRisk;
use crate::think::Think;
use anyhow::Result;
use rig::agent::Agent;
use rig::providers::anthropic::completion::CompletionModel as AnthropicCompletionModel;
use rig::providers::gemini::completion::CompletionModel as GeminiCompletionModel;

use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Features {
    pub autonomous: bool,
    pub deep_research: bool,
}

pub async fn create_solana_agent(
    preamble: Option<String>,
    features: Features,
) -> Result<Agent<AnthropicCompletionModel>> {
    let preamble = preamble.unwrap_or(format!(
        "{} {}",
        "you are a solana trading agent that can also interact with pump.fun;",
        PREAMBLE_COMMON
    ));

    let mut agent = claude_agent_builder()
        .preamble(&preamble)
        .tool(GetQuote)
        .tool(GetSolBalance)
        .tool(GetSplTokenBalance)
        .tool(SearchOnDexScreener)
        .tool(FetchTopTokens)
        .tool(FetchTokenPrice)
        .tool(DeployPumpFunToken)
        .tool(FetchTokenMetadata)
        .tool(ResearchXProfile)
        .tool(ViewImage)
        .tool(FetchXPost)
        .tool(SearchTweets)
        .tool(AnalyzeRisk)
        .tool(FetchPriceActionAnalysis)
        .tool(Think)
        .tool(GetCurrentTime)
        .tool(SearchWeb)
        .tool(AnalyzePageContent)
        .tool(AnalyzeSentiment);

    if features.autonomous {
        agent = agent.tool(Swap).tool(CreateAdvancedOrder);
    }

    Ok(agent.build())
}

pub fn create_solana_agent_gemini(
    preamble: Option<String>,
    features: Features,
) -> Agent<GeminiCompletionModel> {
    let preamble = preamble.unwrap_or(format!(
        "{} {}",
        "you are a solana trading agent that can also interact with pump.fun;",
        PREAMBLE_COMMON
    ));

    let mut agent = gemini_agent_builder()
        .preamble(&preamble)
        .tool(GetQuote)
        .tool(GetSolBalance)
        .tool(GetSplTokenBalance)
        .tool(SearchOnDexScreener)
        .tool(FetchTopTokens)
        .tool(DeployPumpFunToken)
        .tool(FetchTokenMetadata)
        .tool(ResearchXProfile)
        .tool(FetchXPost)
        .tool(SearchTweets)
        .tool(AnalyzeRisk)
        .tool(FetchPriceActionAnalysis)
        .tool(Think)
        .tool(AnalyzeHolderDistribution)
        .tool(AnalyzeSentiment)
        .tool(GetCurrentTime)
        .tool(SearchWeb)
        .tool(ViewImage)
        .tool(AnalyzePageContent);

    if features.autonomous {
        agent = agent.tool(Swap).tool(CreateAdvancedOrder);
    }

    agent.build()
}
