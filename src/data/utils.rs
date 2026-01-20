use std::collections::HashSet;

use poise::serenity_prelude as serenity;

// owners can NOT be changed at runtime so u need to restart bot to add new owners.
pub fn get_owners() -> HashSet<serenity::UserId>
{
    [1166753301153448058u64]
        .iter()
        .map(|v| serenity::UserId::new(*v))
        .collect()
}
