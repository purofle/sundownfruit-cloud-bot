use anyhow::Result;
use teloxide::{
    dispatching::DpHandlerDescription,
    dptree::{self, Handler},
    prelude::{DependencyMap, Requester},
    types::Message,
    Bot,
};

use crate::{manager::service_quotas::ServiceQuotas, BotState, MyDialogue};

pub fn set_create_vm_state(
    handler: Handler<'_, DependencyMap, Result<()>, DpHandlerDescription>,
) -> Handler<'_, DependencyMap, Result<()>, DpHandlerDescription> {
    handler
        .branch(dptree::case![BotState::ReceiveName].endpoint(receive_name))
        .branch(dptree::case![BotState::ReceiveCPUs { name }].endpoint(receive_cpu))
}

async fn receive_name(bot: Bot, dialogue: MyDialogue, msg: Message) -> Result<()> {
    match msg.text() {
        Some(text) => {
            dialogue
                .update(BotState::ReceiveCPUs { name: text.into() })
                .await?;

            let _service_quotas = ServiceQuotas::get_from_db(msg.chat.id.0).await;

            bot.send_message(
                msg.chat.id,
                "How many CPU cores would you like to allocate?",
            )
            .await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Please enter a valid name for your VM.")
                .await?;
        }
    }

    Ok(())
}

async fn receive_cpu(bot: Bot, dialogue: MyDialogue, msg: Message, name: String) -> Result<()> {
    match msg.text().map(|text| text.parse::<u8>()) {
        Some(Ok(cpus)) => {
            bot.send_message(
                msg.chat.id,
                "How much Disk space would you like to allocate?",
            )
            .await?;

            dialogue
                .update(BotState::ReceiveMemory { name, cpus })
                .await?;
            /*  */
        }
        _ => {
            bot.send_message(msg.chat.id, "Please enter a valid number of CPU cores.")
                .await?;
        }
    }

    Ok(())
}
