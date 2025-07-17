use expense_tracker:: {
    types::{ExpenseCategory, UserSession, TAX_RATE, MAX_DAILY_EXPENSES},
    expense::{create_expense, create_expense_from_tuple, process_expense, categorize_expense_amount},
    calculator::{calculate_tax, WeeklySummary},
    display::*,
};
use personal_expense_tracker::{display::{print_budget_tracking, print_final_summary, print_session_info}, expense::{self, create_expense_from_tuple}, types::UserSession, WeeklySummary};

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

    let store_name: &str = "SuperMart";
    let payment_method = "Credit Card";

    let mut receipt_notes: String = String::new();
    receipt_notes.push_str("Bought: milk, bread, eggs");

    let description = String::from("Grocery shopping");
    let mut full_description = "Weekly".to_string();
    full_description.push_str("grocery shoppping at");
    full_description.push_str(store_name);

    print_strig_examples(store_name, receipt_notes);
    println!("   Full description: {}", full_description);

    let base_amount = 500.0;
    let mut running_total = 0.0;
    let mut transaction_count = 0;

    let (total_spent, count) = process_expenses(&expenses);
    running_total = total_spent;
    transaction_count = count;
    session.expense_count = count as i32;

    let tax_amount = calculate_tax(running_total);
    print_running_totals(base_amount, running_total, transaction_count, tax_amount);
    print_budget_tracking(base_amount, running_total);
    print_type_examples();

    let weekly_summary = WeeklySummary::new(daily_totals);
    print_weekly_summary(&weekly_summary);

    println!("\n🏪 System constants:");
    println!("   Tax rate: {:.1}%", TAX_RATE * 100.0);
    println!("   Max daily expenses: {}", MAX_DAILY_EXPENSES);

    println!("\n🏷️  Expense categorization:");
    for expense in &expenses{
        let category_desc = categorize_expense_amount(expense.amount);
        println!("   ${:.2} - {}", expense.amount, category_desc);
    }

    let summary_data = (
        transaction_count,
        running_total,
        tax_amount,
        "expenses",
        '✓'                   
    );

    print_final_summary(
        summary_data.0, 
        summary_data.1, 
        summary_data.2, 
        summary_data.4
    );

    let weekly_budget = 350.0;
    println!("   Weekly status: {}", weekly_summary.budget_status(weekly_budget));
}