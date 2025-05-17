use serde::{Deserialize, Serialize};
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
                let mut engine = rhai::Engine::new();
                let mut scope = rhai::Scope::new();
                
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