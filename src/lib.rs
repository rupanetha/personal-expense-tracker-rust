pub mod types;
pub mod expense;
pub mod calculator;
pub mod display;

pub use types::{ExpenseData, ExpenseCategory, TAX_RATE, MAX_DAILY_EXPENSES, UserSession};
pub use expense::{create_expense, create_expense_from_tuple, process_expenses, categorize_expense_amount};
pub use calculator::{calculate_tax, calculate_total_with_tax, WeeklySummary};
pub use display::{print_header, print_session_info, print_expense_details, print_string_examples, 
                  print_running_totals, print_budget_tracking, print_weekly_summary, 
                  print_final_summary, print_type_examples};