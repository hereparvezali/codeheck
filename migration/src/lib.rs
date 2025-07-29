pub use sea_orm_migration::prelude::*;

mod m20250729_154307_users;
mod m20250729_165841_problems;
mod m20250729_194923_testcases;
mod m20250729_201113_contests;
mod m20250729_202337_submissions;
mod m20250729_202925_contest_problems;
mod m20250729_204136_contest_registrations;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250729_154307_users::Migration),
            Box::new(m20250729_165841_problems::Migration),
            Box::new(m20250729_194923_testcases::Migration),
            Box::new(m20250729_201113_contests::Migration),
            Box::new(m20250729_202337_submissions::Migration),
            Box::new(m20250729_202925_contest_problems::Migration),
            Box::new(m20250729_204136_contest_registrations::Migration),
        ]
    }
}
