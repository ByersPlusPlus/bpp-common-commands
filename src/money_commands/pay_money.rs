use bpp_command_api::{CommandError, structs::{Message, ServiceDirectory}, traits::Command, userservice::{BppUser, BppUserFilter, BppUserFilters, BppUsers, bpp_user_filter::Filter, bpp_user_filters::SortingFields}};
use tonic::Request;

#[derive(Clone)]
pub struct PayCommand;

#[async_trait]
impl Command for PayCommand {
    async fn execute(&self, message: Message, service_directory: &mut ServiceDirectory) -> Result<(), CommandError> {
        // args:
        // 0: username or id
        // 1: amount

        let args = message.command_args;
        if args.len() != 2 {
            service_directory.youtubeservice_client.send_message(Request::new(format!("@{} Usage: !pay [user] [amount]", message.user.display_name))).await.unwrap();
            return Ok(());
        }

        // first try to get the target user by username
        let mut target_user = service_directory.userservice_client.filter_users(Request::new(
            BppUserFilters {
                sorting: SortingFields::Default.into(),
                filters: vec![
                    BppUserFilter {
                        filter: Some(Filter::Name(args[0].clone()))
                    }
                ]
            }
        )).await.unwrap().into_inner();
        if target_user.count == 0 {
            // if the user wasn't found, try to get it by id
            target_user = service_directory.userservice_client.filter_users(Request::new(
                BppUserFilters {
                    sorting: SortingFields::Default.into(),
                    filters: vec![
                        BppUserFilter {
                            filter: Some(Filter::ChannelId(args[0].clone()))
                        }
                    ]
                }
            )).await.unwrap().into_inner();

            // if the user wasn't found, send an error message
            if target_user.count == 0 {
                service_directory.youtubeservice_client.send_message(Request::new(format!("@{} I couldn't find that user!", message.user.display_name))).await.unwrap();
                return Ok(());
            }
        }
        
        let mut target_user = target_user.users.get_mut(0).unwrap();
        let amount = args[1].parse::<i64>().unwrap();
        let mut source_user: BppUser = message.user.into();

        // Check if the user has enough money
        if (source_user.money as i64) < amount {
            service_directory.youtubeservice_client.send_message(Request::new(format!("@{} You don't have enough money!", source_user.display_name))).await.unwrap();
            return Ok(());
        }

        target_user.money += amount as f64;
        source_user.money -= amount as f64;

        // Update the users
        service_directory.userservice_client.update_users(Request::new(BppUsers {
            users: vec![
                target_user.clone(),
                source_user.clone()
            ],
            count: 2
        })).await.unwrap();
        
        service_directory.youtubeservice_client.send_message(Request::new(format!("@{} gave @{} Boondollars to {}.", source_user.display_name, amount, target_user.display_name))).await.unwrap();
        Ok(())
    }
}