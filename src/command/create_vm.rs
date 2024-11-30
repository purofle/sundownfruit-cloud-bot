use anyhow::Result;
use teloxide::{dispatching::DpHandlerDescription, dptree::{self, Handler}, prelude::{DependencyMap, Requester}, types::Message, Bot};

use crate::{BotState, MyDialogue};

pub fn set_create_vm_state(handler: Handler<'_, DependencyMap, Result<()>, DpHandlerDescription>) -> Handler<'_, DependencyMap, Result<()>, DpHandlerDescription> {
    handler.branch(dptree::case![BotState::ReceiveName].endpoint(receive_name))
    .branch(dptree::case![BotState::ReceiveCPUs].endpoint(receive_cpu))
}

async fn receive_name(bot: Bot, dialogue: MyDialogue, msg: Message) -> Result<()> {

    bot.send_message(msg.chat.id, "How many CPU cores would you like to allocate?").await?;

    dialogue.update(BotState::ReceiveCPUs).await?;

    Ok(())
}

async fn receive_cpu(bot: Bot, dialogue: MyDialogue, msg: Message) -> Result<()> {

    bot.send_message(msg.chat.id, "How much Disk space would you like to allocate?").await?;

    dialogue.update(BotState::ReceiveDiskSize).await?;

    Ok(())
}