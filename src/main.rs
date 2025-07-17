use expense_tracker:: {
    types::{ExpenseCategory, UserSession, TAX_RATE, MAX_DAILY_EXPENSES},
    expense::{create_expense, create_expense_from_tuple, process_expense, categorize_expense_amount},
    calculator::{calculate_tax, WeeklySummary},
    display::*,
};
use personal_expense_tracker::{display::print_session_info, expense::{self, create_expense_from_tuple}, types::UserSession};

fn main() {
    print_header();

    let mut session = UserSession::new(12345_u64, true);
    print_session_info(&session);

    let coffee_price: f64 = 4.50;
    let gas_price = 3.89_f32;
    let food_category: char = 'F';
    let has_receipt = false;

    println!("\n💰 Processing expenses:");

    let expense_tuple1: (f64, &str, char) = (15.99, "lunch", 'F');
    let expense_tuple2 = (45.00, "gas", 'T');
    let expense_tuple3 = (12.50, "coffee", food_category);

    let expense1 = create_expense_from_tuple(expense_tuple1).unwrap();
    let expense2 = create_expense_from_tuple(expense_tuple2).unwrap();
    let expense3 = create_expense_from_tuple(expense_tuple3).unwrap();
    let expense4 = create_expense(
        89.99,
        "weekly groceries",
        ExpenseCategory::Food,
        "SuperMart",
        "Credit Card"
    );

    let daily_totals: [f64; 7] = [45.50, 32.10, 67.25, 28.90, 55.75, 89.30, 41.20];
    
    let expenses = vec![expense1, expense2, expense3, expense4];
    for (i, expense) in expenses.iter().enumerate() {
        print_expense_details(expense, i);
    }
}