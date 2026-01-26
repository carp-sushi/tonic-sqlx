use crate::{
    Error, Result,
    domain::{StoryId, TaskId},
};
use uuid::Uuid;

const MAX_STR_LEN: usize = 1000;
const MIN_PAGE_LIMIT: i64 = 10;
const MAX_PAGE_LIMIT: i64 = 100;

/// Common validator for story and task names.
pub struct NameValidator;

impl NameValidator {
    /// Validates name length (0 < name.len() < 1000).
    pub fn validate<S: Into<String>>(name: S) -> Result<String> {
        let name = name.into().trim().to_string();
        if name.is_empty() {
            return Err(Error::invalid_args("name cannot be empty"));
        }
        if name.len() > MAX_STR_LEN {
            return Err(Error::invalid_args("name is too long"));
        }
        Ok(name)
    }

    /// Validates an optional name if provided.
    pub fn validate_optional<S: Into<String>>(maybe_name: Option<S>) -> Result<Option<String>> {
        if let Some(name) = maybe_name {
            let validated_name = NameValidator::validate(name)?;
            return Ok(Some(validated_name));
        }
        Ok(None)
    }
}

/// Ensure a story id value can be created from a string
pub(crate) fn validate_story_id(input: &str) -> Result<StoryId> {
    let uuid = validate_uuid(input)?;
    Ok(StoryId(uuid))
}

/// Ensure a task id value can be created from a string
pub(crate) fn validate_task_id(input: &str) -> Result<TaskId> {
    let uuid = validate_uuid(input)?;
    Ok(TaskId(uuid))
}

/// Ensure a uuid value can be created from a string
fn validate_uuid(value: &str) -> Result<Uuid> {
    let value = value.trim().to_lowercase();
    let uuid = Uuid::parse_str(&value).map_err(|err| Error::invalid_args(err.to_string()))?;
    Ok(uuid)
}

/// Ensure a paging params are within reasonable bounds.
pub fn clamp_page_bounds(cursor: i64, limit: i64) -> (i64, i64) {
    let cursor = cursor.clamp(1, i64::MAX);
    let limit = limit.clamp(MIN_PAGE_LIMIT, MAX_PAGE_LIMIT);
    (cursor, limit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_string_success() {
        let result = NameValidator::validate(" test ").unwrap();
        assert_eq!(result, "test");
    }

    #[test]
    fn whitespace_only_fail() {
        assert!(NameValidator::validate("  ").is_err());
        assert!(NameValidator::validate("\t\t").is_err());
        assert!(NameValidator::validate("\n\n").is_err());
    }

    #[test]
    fn max_len_fail() {
        let input = "0123456789!".repeat(100);
        assert!(NameValidator::validate(&input).is_err());
    }

    #[test]
    fn validate_uuid_success() {
        let input = format!(" {} ", Uuid::new_v4().to_string());
        let result = validate_uuid(&input).unwrap();
        assert_eq!(result.to_string(), input.trim());
    }

    #[test]
    fn validate_uuid_fail() {
        assert!(validate_uuid("4ac0160a").is_err());
    }
}
