use crate::expense;
use crate::types::{ExpenseData, UserSession};
use crate::calculator:: WeeklySummary;

pub fn print_header() {
    println!("🛒 Personal Expense Tracker");
    println!("============================");
}

pub fn print_session_info(session: &UserSession) {
    println!("📊 Today's expense tracking:");
    println!("   User ID: {}", session.user_id);
    println!("   Expenses to process: {}", session.expense_count);
    println!("   Weekend surcharge applies: {}", session.is_weekend);
    println!("   Days this month: {}", session.days_this_month);
}

pub fn print_expense_details(expense: &ExpenseData, index: usize) {
    println!("   Expense {}: ${:.2} for {} (category: {} - {})", 
             index + 1,
             expense.amount, 
             expense.description,
             expense.category.to_char(),
             expense.category.name());
    
    if expense.vendor != "Unknown" {
        println!("     Vendor: {}", expense.vendor);
    }
    if expense.payment_method != "Unknown" {
        println!("     Payment: {}", expense.payment_method);
    }
}

pub fn print_string_examples(store_name: &str, receipt_notes: &str) {
    println!("\n🧾 Receipt details:");
    println!("   Store: {}", store_name);
    println!("   Notes: {}", receipt_notes);

    if receipt_notes.len() >= 6 {
        let partial = &receipt_notes[0..6];
        println!("   Partial notes: {}...", partial);
    }
}

pub fn print_running_totals(
    base_amount: f64,
    running_total: f64,
    transaction_count: usize,
    tax_amount: f64
) {
    println!("\n📈 Running calculations:");
    println!("   Base budget: ${:.2}", base_amount);
    println!("   Current total: ${:.2}", running_total);
    println!("   Transactions: {}", transaction_count);
    println!("   Tax: ${:.2}", tax_amount);
    println!("   Total with tax: ${:.2}", running_total + tax_amount);
}

pub fn print_budget_tracking(original_budget: f64, spent: f64) {
    println!("\n💳 Budget tracking:");
    println!("   Original budget: ${:.2}", original_budget);

    let remaining_budget = original_budget - spent;
    println!("   After expenses: ${:.2}", remaining_budget);

    let budget_status = if remaining_budget > 0.0 {
        "Under budget"
    } else {
        "Over budget"
    };
    println!("   Status: {}", budget_status);
}

pub fn print_weekly_summary(summary: &WeeklySummary) {
    println!("\n📊 Weekly summary:");
    println!("   Daily totals: {:?}", summary.daily_totals);
    println!("   Week total: ${:.2}", summary.week_total);
    println!("   Daily average: ${:.2}", summary.daily_average);
    println!("   Highest day: ${:.2}", summary.highest_day);
    println!("   Lowest day: ${:.2}", summary.lowest_day);
}

pub fn print_final_summary(
    transaction_count: usize,
    total: f64,
    tax: f64,
    status_char: char
) {
    println!("\n✅ Summary: {} Daily expenses processed, total: ${:.2} + ${:.2} tax {}", 
             status_char, transaction_count, total, tax);
}

pub fn print_type_examples() {
    println!("\n🧮 Type inference examples:");

    let auto_price = 25.99;
    let auto_quantity = 3;
    let auto_total = auto_price * auto_quantity as f64;

    println!("   Auto-inferred price: ${:.2} (f64)", auto_price);
    println!("   Auto-inferred quantity: {} (i32)", auto_quantity);
    println!("   Calculated total: ${:.2}", auto_total);
}