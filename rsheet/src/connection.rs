use rsheet_lib::command::Command;
use rsheet_lib::connect::{ReadMessageResult, Reader, WriteMessageResult, Writer};
use rsheet_lib::replies::Reply;
use std::error::Error;
use std::sync::{Arc, Mutex};

use crate::commands::handle_command;
use crate::spreadsheet::Spreadsheet;

pub fn handle_connection<R, W>(
    mut reader: R,
    mut writer: W,
    spreadsheet: Arc<Mutex<Spreadsheet>>,
    sender: std::sync::mpsc::Sender<(String, u64)>,
) -> Result<(), Box<dyn Error + Send + Sync>>
where
    R: Reader,
    W: Writer,
{
    loop {
        match reader.read_message() {
            ReadMessageResult::Message(msg) => {
                let command_result = msg.parse::<Command>();

                match command_result {
                    Ok(command) => {
                        let reply_opt = handle_command(&spreadsheet, &sender, command);
                        if let Some(reply) = reply_opt {
                            match writer.write_message(reply) {
                                WriteMessageResult::Ok => {
                                    // Message successfully sent, continue.
                                }
                                WriteMessageResult::ConnectionClosed => {
                                    break;
                                }
                                WriteMessageResult::Err(e) => {
                                    return Err(e.into());
                                }
                            }
                        }
                    }
                    Err(e) => {
                        let reply = Reply::Error(e.to_string());
                        match writer.write_message(reply) {
                            WriteMessageResult::Ok => {
                                // Message successfully sent, continue.
                            }
                            WriteMessageResult::ConnectionClosed => {
                                break;
                            }
                            WriteMessageResult::Err(e) => {
                                return Err(e.into());
                            }
                        }
                    }
                }
            }
            ReadMessageResult::ConnectionClosed => {
                break;
            }
            ReadMessageResult::Err(e) => {
                return Err(e.into());
            }
        }
    }
    Ok(())
}
