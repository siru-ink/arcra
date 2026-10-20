#[derive(Debug, thiserror::Error)]
pub enum ServiceCategoryListAllError {
    #[error("Failed to aquire a database connection.")]
    DBConnectionFailed(#[from] sqlx::Error),
    #[error("Failed to retrieve records from database.")]
    DBRetrievalFailed(#[from] crate::db::ListAllCategoriesError),
}

impl crate::services::Butler {
    pub async fn list_all_categories(
        &self,
    ) -> Result<Vec<crate::domain::Category>, ServiceCategoryListAllError> {
        let mut conn = self.dbase.acquire().await?;
        let categories = crate::db::Db::list_all_categories(&mut conn).await?;
        Ok(categories)
    }
}
