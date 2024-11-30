pub mod command;

use command::create_vm::{set_create_vm_state, CreateVMState};
use dptree::case;
use log::info;
use anyhow::Result;
use teloxide::{dispatching::{dialogue::{self, InMemStorage}, UpdateHandler}, prelude::*, utils::command::BotCommands};
extern crate pretty_env_logger;

type MyDialogue = Dialogue<BotState, InMemStorage<BotState>>;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
enum SimpleCommand {
    Help,
}

#[derive(Clone, Default)]
pub enum BotState {
    #[default]
    DefaultState,
    
    ReceiveName,
    ReceiveMemory,
    ReceiveCPUs,
    ReceiveDiskSize,
    ReceiveImage,
    ReceiveNetwork,

    ReceivePortForwarding,
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
enum Command {
    Help,
    Start,
    Create,
    Cancel,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    info!("Starting command bot...");

    let bot = Bot::from_env();

    Dispatcher::builder(bot, schema())
    .dependencies(dptree::deps![InMemStorage::<BotState>::new()])
    .build()
    .dispatch()
    .await;
}

fn schema() -> UpdateHandler<anyhow::Error> {
    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(
            case![BotState::DefaultState]
            .branch(case![Command::Help].endpoint(help))
            .branch(case![Command::Start].endpoint(start))
                .branch(case![Command::Create].endpoint(create))
        )
        .branch(case![Command::Cancel].endpoint(cancel));

    let message_handler = Update::filter_message()
        .branch(command_handler);

    let handler = set_create_vm_state(message_handler);

        dialogue::enter::<Update, InMemStorage<BotState>, BotState, _>()
        .branch(handler)
}

async fn start(bot: Bot, _: MyDialogue, msg: Message) -> Result<()> {
    bot.send_message(msg.chat.id, "Welcome to use Sundownfruit Cloud!").await?;
    Ok(())
}

async fn help(bot: Bot, msg: Message) -> Result<()> {
    bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
    Ok(())
}

async fn create(bot: Bot, dialogue: MyDialogue, msg: Message) -> Result<()> {
    bot.send_message(msg.chat.id, "Initiating VM creation...").await?;
    bot.send_message(msg.chat.id, "What would you like to name your VM?").await?;
    dialogue.update(BotState::ReceiveCPUs).await?;
    Ok(())
}

async fn cancel(bot: Bot, dialogue: MyDialogue, msg: Message) -> Result<()> {
    bot.send_message(msg.chat.id, "Cancelling the dialogue.").await?;
    dialogue.exit().await?;
    Ok(())
}
