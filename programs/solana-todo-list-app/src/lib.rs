use anchor_lang::prelude::*;

declare_id!("ib2VcJC9hEdyeEPsJT9NYryjVTGH41kU6u1zsnWR19q");

#[program]
pub mod solana_todo_list_app {
    use super::*;

    pub fn adding_task(ctx: Context<AddingTask>, text: String) -> Result<()> {
        let task = &mut ctx.accounts.task;
        let author = &ctx.accounts.author; // The `author` account
        let clock = Clock::get().unwrap(); /// Getting the current timestamp
        if text.chars().count() > 400 {
            return Err(ErrorCode::TextTooLong.into());
        }

        task.author = *author.key;
        task.is_done = false;
        task.created_at = clock.unix_timestamp;
        task.updated_at = clock.unix_timestamp;
        task.text = text;
        Ok(())
    }

    pub fn updating_task(ctx: Context<UpdatingTask>, is_done: bool) -> Result<()> {
        let task = &mut ctx.accounts.task;
        let author = &ctx.accounts.author; // The `author` account
        let clock = Clock::get().unwrap(); // Getting the current timestamp
        task.author = *author.key;
        task.is_done = is_done;
        task.updated_at = clock.unix_timestamp;
        Ok(())
        }

        pub fn deleting_task(ctx: Context<DeletingTask>) -> Result<()> {
            let task = &mut ctx.accounts.task;
            let author = &ctx.accounts.author; // The `author` account
            let clock = Clock::get().unwrap(); // Getting the current timestamp
            task.author = *author.key;
            task.is_done = true;
            task.updated_at = clock.unix_timestamp;
            Ok(())
            }
            #[derive(Accounts)]
            pub struct DeletingTask<'info> {
            #[account(mut, has_one = author)]
            pub task: Account<'info, Task>,
            pub author: Signer<'info>,
            }
}

#[derive(Accounts)]
pub struct AddingTask<'info> {
    #[account(init, payer= signer, space = Task::LEN)]
    pub task: Account<'info, Task>,
    #[account(mut)]
    pub autho: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct UpdatingTask<'info> {
#[account(mut, has_one = author)]
pub task: Account<'info, Task>,
pub author: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
#[msg("The text is too long")]
TextTooLong,
}