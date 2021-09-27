use bpp_command_api::{CommandError, structs::{Message, ServiceDirectory}, traits::Command, userservice::{BppUserFilter, BppUserFilters, bpp_user_filters::SortingFields}};
use float_ord::FloatOrd;
use tonic::Request;
use async_trait::async_trait;

#[derive(Clone)]
pub struct BoonDollarCommand;

#[async_trait]
impl Command for BoonDollarCommand {
    async fn execute(&self, message: Message, service_directory: &mut ServiceDirectory) -> Result<(), CommandError> {
        let user_info = message.user;
        let channel_id = user_info.channel_id;
        let hours: f64 = user_info.active_time as f64 / 3600.0;
        let money = user_info.money;
        let rank = user_info.rank;

        let filters: Vec<BppUserFilter> = Vec::new();
        let mut users = service_directory.userservice_client.filter_users(Request::new(BppUserFilters {
            filters,
            sorting: SortingFields::HoursDesc.into()
        })).await.unwrap().into_inner().users;
        let hour_rank: u64 = users.iter().position(|u| u.channel_id == channel_id.clone()).unwrap() as u64 + 1;
        users.sort_by(|a, b| {
            let a_ord = FloatOrd(a.money);
            let b_ord = FloatOrd(b.money);

            b_ord.cmp(&a_ord)
        });
        let money_rank: u64 = users.iter().position(|u| u.channel_id == channel_id.clone()).unwrap() as u64 + 1;

        let response_message = format!("{} - Hours: {:.2} (Rank #{}) - Boondollars: {} (Rank #{}) - Echeladder: {} • Next rung in [[Hyperlink blocked]] hours.",
            user_info.display_name,
            hours,
            hour_rank,
            money as u64,
            money_rank,
            rank
        );
        service_directory.youtubeservice_client.send_message(Request::new(response_message)).await.unwrap();

        return Ok(());
    }
}