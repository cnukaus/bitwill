use serde::{Deserialize, Serialize};
use rhai::{Engine, Scope};
use thiserror::Error;
use std::collections::HashMap;

#[derive(Debug, Error)]
pub enum SpendingError {
    #[error("Invalid growth rate: {0}")]
    InvalidGrowthRate(String),
    #[error("Custom formula error: {0}")]
    FormulaError(String),
    #[error("Invalid projection years: {0}")]
    InvalidYears(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSpend {
    pub project_name: String,
    pub daily_spend: f64,
    pub growth_rate: f64,  // e.g., 0.05 for 5% annual
    pub growth_type: GrowthType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GrowthType {
    Compound,
    Flat,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserModel {
    pub user_id: String,
    pub projects: Vec<ProjectSpend>,
    pub projection_years: u32,
}

impl ProjectSpend {
    pub fn new(name: String, daily_spend: f64, growth_rate: f64, growth_type: GrowthType) -> Result<Self, SpendingError> {
        if growth_rate < -1.0 {
            return Err(SpendingError::InvalidGrowthRate("Growth rate cannot be less than -100%".into()));
        }
        
        Ok(Self {
            project_name: name,
            daily_spend,
            growth_rate,
            growth_type,
        })
    }

    pub fn calculate_yearly_spend(&self, year: u32) -> Result<f64, SpendingError> {
        let yearly_base = self.daily_spend * 365.0;
        
        match &self.growth_type {
            GrowthType::Compound => {
                Ok(yearly_base * (1.0 + self.growth_rate).powi(year as i32))
            }
            GrowthType::Flat => {
                Ok(yearly_base * (1.0 + (self.growth_rate * year as f64)))
            }
            GrowthType::Custom(formula) => {
                let mut engine = Engine::new();
                let mut scope = Scope::new();
                
                scope.push("base", yearly_base);
                scope.push("rate", self.growth_rate);
                scope.push("year", year as i64);
                
                engine.eval_with_scope::<f64>(&mut scope, formula)
                    .map_err(|e| SpendingError::FormulaError(e.to_string()))
            }
        }
    }
}

impl UserModel {
    pub fn new(user_id: String, projection_years: u32) -> Result<Self, SpendingError> {
        if projection_years == 0 {
            return Err(SpendingError::InvalidYears("Projection years must be greater than 0".into()));
        }
        
        Ok(Self {
            user_id,
            projects: Vec::new(),
            projection_years,
        })
    }

    pub fn add_project(&mut self, project: ProjectSpend) {
        self.projects.push(project);
    }

    pub fn calculate_total_spend(&self) -> Result<HashMap<u32, f64>, SpendingError> {
        let mut yearly_totals = HashMap::new();
        
        for year in 0..self.projection_years {
            let mut year_total = 0.0;
            
            for project in &self.projects {
                year_total += project.calculate_yearly_spend(year)?;
            }
            
            yearly_totals.insert(year, year_total);
        }
        
        Ok(yearly_totals)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compound_growth() {
        let project = ProjectSpend::new(
            "Test".into(),
            100.0,
            0.05,
            GrowthType::Compound
        ).unwrap();
        
        let year_1 = project.calculate_yearly_spend(1).unwrap();
        assert!((year_1 - 38325.0).abs() < 0.1); // 100 * 365 * 1.05
    }

    #[test]
    fn test_flat_growth() {
        let project = ProjectSpend::new(
            "Test".into(),
            100.0,
            0.05,
            GrowthType::Compound
        ).unwrap();
        
        let year_1 = project.calculate_yearly_spend(1).unwrap();
        let year_2 = project.calculate_yearly_spend(2).unwrap();
        
        assert!(year_2 > year_1);
    }

    #[test]
    fn test_custom_formula() {
        let project = ProjectSpend::new(
            "Test".into(),
            100.0,
            0.05,
            GrowthType::Custom("base * (1 + rate * year) * 1.1".into())
        ).unwrap();
        
        let result = project.calculate_yearly_spend(1);
        assert!(result.is_ok());
    }
}

fn main() {
    // Example usage
    let mut user = UserModel::new("user1".into(), 5).unwrap();
    
    // Add some projects
    let project1 = ProjectSpend::new(
        "Project A".into(),
        100.0,
        0.05,
        GrowthType::Compound
    ).unwrap();
    
    let project2 = ProjectSpend::new(
        "Project B".into(),
        50.0,
        0.03,
        GrowthType::Flat
    ).unwrap();
    
    let project3 = ProjectSpend::new(
        "Project C".into(),
        75.0,
        0.04,
        GrowthType::Custom("base * (1 + rate * year) * 1.1".into())
    ).unwrap();
    
    user.add_project(project1);
    user.add_project(project2);
    user.add_project(project3);
    
    // Calculate and print total spend for each year
    match user.calculate_total_spend() {
        Ok(totals) => {
            for (year, total) in totals {
                println!("Year {}: ${:.2}", year, total);
            }
        }
        Err(e) => eprintln!("Error calculating spend: {}", e),
    }
} 