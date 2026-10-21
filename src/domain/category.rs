#[derive(Debug, thiserror::Error)]
pub enum CategoryIDConstructorError {
    #[error("The provided id must be a positive integer.")]
    InvalidID,
}

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

#[derive(serde::Serialize, Clone)]
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

impl TryFrom<i64> for CategoryID {
    type Error = CategoryIDConstructorError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value > 0 {
            Ok(Self(value))
        } else {
            Err(CategoryIDConstructorError::InvalidID)
        }
    }
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

    pub fn update(mut self, new_name: Option<String>, new_description: Option<String>) -> Self {
        if let Some(name) = new_name.filter(|s| !s.is_empty()) {
            self.name = name;
        }
        if let Some(description) = new_description.filter(|s| !s.is_empty()) {
            self.description = description;
        }
        self
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
