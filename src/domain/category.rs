#[derive(Debug, thiserror::Error)]
pub enum NewCategoryContructorError {
    #[error("The provided name must not be empty.")]
    InvalidName,
    #[error("The provided description must not be empty.")]
    InvalidDescription,
}

#[derive(Debug, thiserror::Error)]
pub enum CategoryHydrateError {
    #[error("The provided id must be a positive integer.")]
    InvalidID,
    #[error("The provided name must not be empty.")]
    InvalidName,
    #[error("The provided description must not be empty.")]
    InvalidDescription,
}

#[derive(serde::Serialize)]
pub struct Category {
    id: i64,
    name: String,
    description: String,
}

pub struct CategoryID(i64);

pub struct NewCategory {
    name: String,
    description: String,
}

impl Category {
    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn try_hydrate(
        id: i64,
        name: String,
        description: String,
    ) -> Result<Self, CategoryHydrateError> {
        if id <= 0 {
            return Err(CategoryHydrateError::InvalidID);
        }
        if name.is_empty() {
            return Err(CategoryHydrateError::InvalidName);
        }
        if description.is_empty() {
            return Err(CategoryHydrateError::InvalidDescription);
        }

        Ok(Self {
            id,
            name,
            description,
        })
    }
}

impl CategoryID {
    pub fn id(&self) -> i64 {
        self.0
    }
}

impl NewCategory {
    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn new(
        name: String,
        description: String,
    ) -> Result<NewCategory, NewCategoryContructorError> {
        if name.is_empty() {
            return Err(NewCategoryContructorError::InvalidName);
        } else if description.is_empty() {
            return Err(NewCategoryContructorError::InvalidDescription);
        }
        Ok(Self { name, description })
    }
}
