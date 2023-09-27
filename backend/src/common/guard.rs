use crate::CONTEXT;

pub fn has_user_guard() -> Result<(), String> {
    CONTEXT.with(|c| {
        let ctx = c.borrow();
        let caller = &ctx.env.caller();
        ctx.user_service
            .get_user(caller)
            .map(|_| ())
            .ok_or_else(|| String::from("UserNotFound"))
    })
}

pub fn user_owner_guard() -> Result<(), String> {
    CONTEXT.with(|c| {
        let ctx = c.borrow();
        let caller = &ctx.env.caller();
        if ctx.user_service.is_owner(caller) {
            Ok(())
        } else {
            let error_message = format!(
                "Highly maybe not register yet! Caller: {:?} is not the owner",
                caller.to_string()
            );
            Err(error_message)
        }
    })
}
