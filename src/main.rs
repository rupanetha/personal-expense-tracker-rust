const TAX_RATE: f64 = 0.08; // 8% sales tax

const MAX_DAILY_EXPENSES: usize = 50;

fn main() {
    println!("🛒 Personal Expense Tracker");
    println!("============================");
    

    let expense_count: i32 = 5;           
    let user_id = 12345_u64;             
    let days_this_month = 31_i32;         
    
    
    let coffee_price: f64 = 4.50;         
    let gas_price = 3.89_f32;             
    
    
    let is_weekend: bool = true;          
    let has_receipt = false;              
    
   
    let food_category: char = 'F';        
    let transport_category = 'T';        
    
    println!("📊 Today's expense tracking:");
    println!("   Expenses to process: {}", expense_count);
    println!("   User ID: {}", user_id);
    println!("   Weekend surcharge applies: {}", is_weekend);
}