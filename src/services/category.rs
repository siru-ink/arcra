use crate::domain;

#[derive(Debug, thiserror::Error)]
pub enum ServiceCategoryListAllError {
    #[error("Failed to aquire a database connection.")]
    DBConnectionFailed(#[from] sqlx::Error),
    #[error("Failed to retrieve records from database.")]
    DBRetrievalFailed(#[from] crate::db::ListAllCategoriesError),
}

#[derive(Debug, thiserror::Error)]
pub enum ServiceCategoryReadByIDError {
    #[error("Failed to construct CategoryID domain object from provided id.")]
    CategoryIDConstructorFailed(#[from] domain::CategoryIDConstructorError),
    #[error("Failed to aquire a database connection.")]
    DBConnectionFailed(#[from] sqlx::Error),
    #[error("Failed to retrieve record from database.")]
    DBRetrievalFailed(#[from] crate::db::ReadCategoryError),
}

#[derive(Debug, thiserror::Error)]
pub enum ServiceCategoryUpdateDataError {
    #[error("Failed to construct CategoryID domain object from provided id.")]
    CategoryIDConstructorFailed(#[from] domain::CategoryIDConstructorError),
    #[error("Failed to aquire a database connection.")]
    DBConnectionFailed(#[from] sqlx::Error),
    #[error("Failed to retrieve record from database.")]
    DBRetrievalFailed(#[from] crate::db::ReadCategoryError),
    #[error("Failed to write updated category data to database record.")]
    DBCategoryUpdateFailed(#[from] crate::db::UpdateCategoryError),
    #[error("No new data to update was provided.")]
    InvalidUpdateData,
}

impl crate::services::Butler {
    pub async fn get_category_by_id(
        &self,
        category_id: i64,
    ) -> Result<domain::Category, ServiceCategoryReadByIDError> {
        let category_id = domain::CategoryID::try_from(category_id)?;
        let mut conn = self.dbase.acquire().await?;
        let category = crate::db::Db::read_category(&mut conn, category_id).await?;
        Ok(category)
    }

    pub async fn list_all_categories(
        &self,
    ) -> Result<Vec<crate::domain::Category>, ServiceCategoryListAllError> {
        let mut conn = self.dbase.acquire().await?;
        let categories = crate::db::Db::list_all_categories(&mut conn).await?;
        Ok(categories)
    }

    pub async fn update_category_data(
        &self,
        category_id: i64,
        new_name: Option<String>,
        new_description: Option<String>,
    ) -> Result<(), ServiceCategoryUpdateDataError> {
        if new_name.is_none() && new_description.is_none() {
            return Err(ServiceCategoryUpdateDataError::InvalidUpdateData);
        }
        let category_id = domain::CategoryID::try_from(category_id)?;
        let mut conn = self.dbase.acquire().await?;
        let category = crate::db::Db::read_category(&mut conn, category_id).await?;
        let category = category.update(new_name, new_description);
        let _ = crate::db::Db::update_category(&mut conn, category).await?;
        Ok(())
    }
}
