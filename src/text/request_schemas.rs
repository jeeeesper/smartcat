use crate::config::prompt::{Message, Prompt};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct OpenAiPrompt {
    pub model: String,
    pub messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct AnthropicPrompt {
    pub model: String,
    pub messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    pub max_tokens: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct DeeplPrompt {
    // TODO: There are more parameters, such as `formality`, `tag_handling` and `source_lang`.
    // They should also be used and configurable.
    pub target_lang: String,
    pub preserve_formatting: bool,
    pub context: String,
    pub text: Vec<String>,
}

impl From<Prompt> for OpenAiPrompt {
    fn from(prompt: Prompt) -> OpenAiPrompt {
        OpenAiPrompt {
            model: prompt
                .model
                .expect("model must be specified either in the api config or in the prompt config"),
            messages: prompt.messages,
            temperature: prompt.temperature,
            stream: prompt.stream,
        }
    }
}

impl From<Prompt> for AnthropicPrompt {
    fn from(prompt: Prompt) -> Self {
        let merged_messages =
            prompt
                .messages
                .into_iter()
                .fold(Vec::new(), |mut acc: Vec<Message>, mut message| {
                    if message.role == "system" {
                        message.role = "user".to_string();
                    }
                    match acc.last_mut() {
                        Some(last_message) if last_message.role == message.role => {
                            last_message.content.push_str("\n\n");
                            last_message.content.push_str(&message.content);
                        }
                        _ => acc.push(message),
                    }
                    acc
                });

        AnthropicPrompt {
            model: prompt.model.expect("model must be specified"),
            messages: merged_messages,
            temperature: prompt.temperature,
            stream: prompt.stream,
            max_tokens: 4096,
        }
    }
}

impl From<Prompt> for DeeplPrompt {
    fn from(prompt: Prompt) -> Self {
        let Some((message, context)) = prompt.messages.split_last() else {
            panic!("No messages for translation specified!");
        };
        DeeplPrompt {
            target_lang: prompt
                .model
                .expect("Target Language ('model') must be specified either in the api config or in the prompt config"),
            text: vec![message.content.to_string()],
            context: context.into_iter().map(|m| m.content.clone()).collect::<Vec<_>>().join("\n"),
            preserve_formatting: true,
         }
    }
}
