use core::f64;

use crate::types::TAX_RATE;

pub fn calculate_tax(amount: f64) -> f64 {
    amount * TAX_RATE
}

pub fn calculate_total_with_tax(amount: f64) -> f64 {
    amount + calculate_tax(amount)
}

pub fn calculate_totals(amounts: &[f64]) -> (f64, f64, f64, f64) {
    if amounts.is_empty() {
        return (0.0, 0.0, 0.0, 0.0);
    }

    let total: f64 = amounts.iter().sum();
    let average = total / amounts.len() as f64;
    let max_amount = amounts.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let min_amount = amounts.iter().fold(f64::INFINITY, |a, &b| a.min(b));

    (total, average, max_amount, min_amount)
}

#[derive(Debug)]
pub struct WeeklySummary {
    pub daily_totals: [f64; 7],
    pub week_total: f64,
    pub daily_average: f64,
    pub highest_day: f64,
    pub lowest_day: f64,
}

impl WeeklySummary {
    pub fn new(daily_totals: [f64; 7]) -> Self {
        let (week_total, daily_average, highest_day, lowest_day) = calculate_totals(&daily_totals);

        Self {
            daily_totals,
            week_total,
            daily_average,
            highest_day,
            lowest_day,
        }
    }

    pub fn is_under_budget(&self, budget: f64) -> bool {
        self.week_total <= budget
    }

    pub fn budget_status(&self, budget: f64) -> &'static str {
        if self.is_under_budget(budget) {
            "Under budget"
        } else {
            "Over budget"
        }
    }
}

pub fn calculate_budget_percentage(spent: f64, budget: f64) -> f64 {
    if budget <= 0.0 {
        100.0
    } else {
        (spent / budget) * 100.0
    }
}