//! Access the notifications portion of the GitHub API
imports!();
use client::{PatchQueryBuilder, Executor};

new_type!(
    Notifications
    Threads
    ID
);

from!(
    @PatchQueryBuilder
        -> Notifications  = "notifications"
    @Notifications
        -> Threads = "threads"
    @Threads
        => ID
    @ID
        => Executor
);

impl_macro!(
    @Notifications
        |=> threads -> Threads
        |
    @Threads
        |
        |=> id -> ID = id_str
    @ID
        |
        |-> execute
);
