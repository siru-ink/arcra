use crate::domain;

#[derive(Debug, thiserror::Error)]
pub enum CreateCategoryError {
    #[error("Sqlx sql execution failed.")]
    SqlFailed(#[from] sqlx::Error),
    #[error("Failed to convert new database row into domain object.")]
    HydrateFailed(#[from] domain::CategoryHydrateError),
}

#[derive(Debug, thiserror::Error)]
pub enum ReadCategoryError {
    #[error("Sqlx sql execution failed.")]
    SqlFailed(#[from] sqlx::Error),
    #[error("No row found for ID in database.")]
    RowNotFound,
    #[error("Failed to convert new database row into domain object.")]
    HydrateFailed(#[from] domain::CategoryHydrateError),
}

#[derive(Debug, thiserror::Error)]
pub enum UpdateCategoryError {
    #[error("Sqlx sql execution failed.")]
    SqlFailed(#[from] sqlx::Error),
    #[error("No rows were modified in the database.")]
    NoRowModified,
}

#[derive(Debug, thiserror::Error)]
pub enum DeleteCategoryError {
    #[error("Sqlx sql execution failed.")]
    SqlFailed(#[from] sqlx::Error),
    #[error("No rows were deleted in the database.")]
    NoRowDeleted,
}

#[derive(Debug, thiserror::Error)]
pub enum ListAllCategoriesError {
    #[error("Sqlx sql execution failed.")]
    SqlFailed(#[from] sqlx::Error),
    #[error("Failed to convert (at least one) database row into domain object.")]
    HydrateFailed(#[from] domain::CategoryHydrateError),
}

struct CategoryRow {
    id: i64,
    name: String,
    description: String,
}

impl TryFrom<CategoryRow> for domain::Category {
    type Error = domain::CategoryHydrateError;
    fn try_from(value: CategoryRow) -> Result<Self, Self::Error> {
        domain::Category::try_hydrate(value.id, value.name, value.description)
    }
}

impl crate::db::Db {
    pub async fn create_category(
        conn: &mut sqlx::SqliteConnection,
        category: domain::NewCategory,
    ) -> Result<domain::Category, CreateCategoryError> {
        let new_row = sqlx::query_as!(
            CategoryRow,
            "INSERT INTO categories (name,description) VALUES (?,?) RETURNING id,name,description",
            category.name(),
            category.description(),
        )
        .fetch_one(conn)
        .await?;
        Ok(new_row.try_into()?)
    }

    pub async fn read_category(
        conn: &mut sqlx::SqliteConnection,
        id: domain::CategoryID,
    ) -> Result<domain::Category, ReadCategoryError> {
        let Some(category) = sqlx::query_as!(
            CategoryRow,
            "SELECT * FROM categories WHERE id = ?",
            id.id()
        )
        .fetch_optional(conn)
        .await?
        else {
            return Err(ReadCategoryError::RowNotFound);
        };
        Ok(category.try_into()?)
    }

    pub async fn update_category(
        conn: &mut sqlx::SqliteConnection,
        category: domain::Category,
    ) -> Result<(), UpdateCategoryError> {
        let result = sqlx::query!(
            "UPDATE categories SET name = ?, description = ? WHERE id = ?",
            category.name(),
            category.description(),
            category.id()
        )
        .execute(conn)
        .await?;

        if result.rows_affected() == 0 {
            return Err(UpdateCategoryError::NoRowModified);
        }

        #[cfg(debug_assertions)]
        if result.rows_affected() > 1 {
            eprintln!(
                "Error: More than 1 row affected in categories update. {} rows were affected. {} id was used.",
                result.rows_affected(),
                category.id()
            )
        }

        Ok(())
    }

    pub async fn delete_category(
        conn: &mut sqlx::SqliteConnection,
        id: domain::CategoryID,
    ) -> Result<(), DeleteCategoryError> {
        let result = sqlx::query!("DELETE FROM categories WHERE id = ?", id.id())
            .execute(conn)
            .await?;

        if result.rows_affected() == 0 {
            return Err(DeleteCategoryError::NoRowDeleted);
        }

        #[cfg(debug_assertions)]
        if result.rows_affected() > 1 {
            eprintln!(
                "Error: More than 1 row affected in categories delete. {} rows were affected. {} id was used.",
                result.rows_affected(),
                id.id()
            )
        }

        Ok(())
    }

    pub async fn list_all_categories(
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<Vec<domain::Category>, ListAllCategoriesError> {
        let categories_rows = sqlx::query_as!(CategoryRow, "SELECT * FROM categories")
            .fetch_all(conn)
            .await?;

        let categories = categories_rows
            .into_iter()
            .map(domain::Category::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(categories)
    }
}
