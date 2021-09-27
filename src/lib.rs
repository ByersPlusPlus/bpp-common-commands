#[macro_use]
extern crate async_trait;

use bpp_command_api::{export_command, traits::CommandRegistrar};

pub mod money_commands;
use money_commands::{check_money::BoonDollarCommand, pay_money::PayCommand};

// Don't touch this macro.
export_command!(register);

// Add your commands here.
// You can register as many as you like.
#[allow(improper_ctypes_definitions)]
extern "C" fn register(registrar: &mut dyn CommandRegistrar) {
    // registrar.register_command("!mycommand", &[], Box::new(MyCommand));
    registrar.register_command("!b$", &[], Box::new(BoonDollarCommand));
    registrar.register_command("!pay", &["!give"], Box::new(PayCommand));
}