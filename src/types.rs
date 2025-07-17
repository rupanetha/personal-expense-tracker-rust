pub const TAX_RATE: f64 = 0.08;
pub const MAX_DAILY_EXPENSES: usize = 50;

#[derive(Debug, Clone)]
pub struct ExpenseData {
    pub amount: f64,
    pub description: String,
    pub category: ExpenseCategory,
    pub vendor: String,
    pub payment_method: String,
}

#[derive(Debug, Clone, Copy)]
pub enum ExpenseCategory {
    Food,
    Transport,
    Entertainment,
    Health,
    Shopping,
}

impl ExpenseCategory {
    pub fn to_char(self) -> char {
        match self{
            ExpenseCategory::Food => 'F',
            ExpenseCategory::Transport => 'T',
            ExpenseCategory::Entertainment => 'E',
            ExpenseCategory::Health => 'H',
            ExpenseCategory::Shopping => 'S',
        }
    }

    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'F' => Some(ExpenseCategory::Food),
            'T' => Some(ExpenseCategory::Transport),
            'E' => Some(ExpenseCategory::Entertainment),
            'H' => Some(ExpenseCategory::Health),
            'S' => Some(ExpenseCategory::Shopping),
            _ => None,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            ExpenseCategory::Food => "Food",
            ExpenseCategory::Transport => "Transport",
            ExpenseCategory::Entertainment => "Entertainment",
            ExpenseCategory::Health => "Health",
            ExpenseCategory::Shopping => "Shopping",
        }
    }
}

pub struct UserSession {
    pub user_id: u64,
    pub expense_count: i32,
    pub is_weekend: bool,
    pub days_this_month: i32,
}

impl UserSession {
    pub fn new(user_id: u64, is_weekend: bool) -> Self {
        Self {
            user_id,
            expense_count: 0,
            is_weekend,
            days_this_month: 31
        }
    }
}