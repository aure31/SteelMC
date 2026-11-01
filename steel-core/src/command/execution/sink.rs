//! Output target for command feedback.
//!
//! Vanilla keeps the feedback contract on a `CommandSource` that is separate
//! from the source stack, so a source that is not an interactive sender (a
//! command block, for example) can capture or drop output without changing how
//! the command itself runs. Steel splits the same way: [`CommandSender`]
//! identifies who started the command, while a [`CommandMessageSink`] decides
//! where its feedback goes.

use steel_registry::vanilla_game_rules::SEND_COMMAND_FEEDBACK;
use text_components::TextComponent;

use crate::command::sender::CommandSender;

/// Where command feedback produced during execution is delivered.
pub(crate) trait CommandMessageSink: Send + Sync {
    /// Delivers one message to the source.
    fn send_system_message(&self, message: &TextComponent);

    /// Whether success feedback reaches the source.
    fn accepts_success(&self) -> bool;

    /// Whether failure feedback reaches the source.
    fn accepts_failure(&self) -> bool;

    /// Whether success feedback requested for broadcast reaches operators.
    fn should_inform_admins(&self) -> bool;
}

/// Delivers feedback through the initiating [`CommandSender`].
pub(crate) struct SenderMessageSink {
    sender: CommandSender,
}

impl SenderMessageSink {
    pub(crate) const fn new(sender: CommandSender) -> Self {
        Self { sender }
    }
}

impl CommandMessageSink for SenderMessageSink {
    fn send_system_message(&self, message: &TextComponent) {
        self.sender.send_message(message);
    }

    fn accepts_success(&self) -> bool {
        // Vanilla gates player success feedback on `sendCommandFeedback` and
        // always accepts it for the server and Rcon sources.
        self.sender
            .get_player()
            .is_none_or(|player| player.get_world().get_game_rule(&SEND_COMMAND_FEEDBACK))
    }

    fn accepts_failure(&self) -> bool {
        true
    }

    fn should_inform_admins(&self) -> bool {
        // Vanilla gates the server and Rcon sources on the
        // `broadcast-console-to-ops` and `broadcast-rcon-to-ops` properties.
        // Steel does not model those properties, so it keeps their vanilla
        // default.
        true
    }
}
