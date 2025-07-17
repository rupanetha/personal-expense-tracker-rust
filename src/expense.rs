use crate::types::{ExpenseData, ExpenseCategory};

pub fn create_expense(amount: f64, description: &str, category: ExpenseCategory, vendor: &str, payment_method: &str) -> ExpenseData {
    ExpenseData { 
        amount,
        description: description.to_string(),
        category, 
        vendor: vendor.to_string(), 
        payment_method: payment_method.to_string(),
    }
}

pub fn create_expense_from_tuple(data: (f64, &str, char)) -> Option<ExpenseData> {
    let category = ExpenseCategory::from_char(data.2)?;

    Some(ExpenseData {
        amount: data.0,
        description: data.1.to_string(),
        category,
        vendor: "Unknown".to_string(),
        payment_method: "Unknown".to_string(),
    })
}

pub fn process_expenses(expenses: &[ExpenseData]) -> (f64, usize) {
    let total_amount: f64 = expenses.iter().map(|e| e.amount).sum();
    let count = expenses.len();
    (total_amount, count)
}

pub fn categorize_expense_amount(amount: f64) -> &'static str {
    match amount {
        x if x < 10.0 => "Small purchase",
        x if x < 50.0 => "Medium purchase",
        x if x < 100.0 => "Large purchase",
        _ => "Major expense"
    }
}

pub fn filter_by_category(expenses: &[ExpenseData], category: ExpenseCategory) -> Vec<&ExpenseData> {
    expenses.iter()
        .filter(|expense| matches!(expense.category, cat if cat as u8 == category as u8))
        .collect()
}

pub fn find_most_expensive(expenses: &[ExpenseData]) -> Option<&ExpenseData> {
    expenses.iter().max_by(|a,b| a.amount.partial_cmp(&b.amount).unwrap())
}
