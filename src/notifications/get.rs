//! Access the notifications portion of the GitHub API
imports!();
use client::{GetQueryBuilder, Executor};

new_type!(
    Notifications
);

from!(
    @GetQueryBuilder
        -> Notifications  = "notifications"
    @Notifications
        => Executor
);

impl_macro!(
    @Notifications
        |        
        |-> execute
);
