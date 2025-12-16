use crate::{
    Error, Result,
    domain::{StoryId, TaskId},
};
use uuid::Uuid;

const MAX_STR_LEN: usize = 1000;
const MIN_PAGE_LIMIT: i64 = 10;
const MAX_PAGE_LIMIT: i64 = 100;

/// Validate a string's length if it is defined.
pub fn validate_optional_string_length(
    maybe_value: Option<String>,
    param_name: &str,
) -> Result<Option<String>> {
    if let Some(value) = maybe_value {
        let validated = validate_string_length(&value, param_name)?;
        return Ok(Some(validated));
    }
    Ok(None)
}

/// Put some reasonable limit on string length.
pub fn validate_string_length(value: &str, param_name: &str) -> Result<String> {
    let value = value.trim().to_string();
    if value.is_empty() {
        let errs = format!("{} cannot be empty", param_name);
        return Err(Error::invalid_args(errs));
    }
    if value.len() > MAX_STR_LEN {
        let errs = format!("{} is too long", param_name);
        return Err(Error::invalid_args(errs));
    }
    Ok(value)
}

/// Ensure a story id value can be created from a string
pub fn validate_story_id(input: &str) -> Result<StoryId> {
    let uuid = validate_uuid(input)?;
    Ok(StoryId(uuid))
}

/// Ensure a task id value can be created from a string
pub fn validate_task_id(input: &str) -> Result<TaskId> {
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
        let result = validate_string_length(" test ", "").unwrap();
        assert_eq!(result, "test");
    }

    #[test]
    fn non_empty_fail() {
        assert!(validate_string_length("  ", "2spaces").is_err());
    }

    #[test]
    fn max_len_fail() {
        let input = "0123456789!".repeat(100);
        assert!(validate_string_length(&input, "").is_err());
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
