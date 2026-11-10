use serde_json::Value;

pub(crate) fn compare(
    actual: Option<&Value>,
    operator: &str,
    expected: Option<&Value>,
) -> Result<bool, String> {
    let exists = actual.is_some();
    if operator == "exists" {
        return Ok(exists);
    }
    if operator == "not_exists" {
        return Ok(!exists);
    }
    let Some(actual) = actual else {
        return Ok(false);
    };
    let expected = expected.ok_or_else(|| format!("filter operator {operator} requires value"))?;
    Ok(match operator {
        "eq" => actual == expected,
        "ne" => actual != expected,
        "lt" | "lte" | "gt" | "gte" => {
            let left = actual
                .as_f64()
                .ok_or_else(|| format!("filter operator {operator} requires numeric values"))?;
            let right = expected
                .as_f64()
                .ok_or_else(|| format!("filter operator {operator} requires numeric values"))?;
            match operator {
                "lt" => left < right,
                "lte" => left <= right,
                "gt" => left > right,
                _ => left >= right,
            }
        }
        "contains" => match (actual, expected) {
            (Value::String(left), Value::String(right)) => left.contains(right),
            (Value::Array(values), expected) => values.contains(expected),
            _ => return Err("contains requires a string/string or array/value pair".into()),
        },
        "starts_with" => actual
            .as_str()
            .zip(expected.as_str())
            .is_some_and(|(left, right)| left.starts_with(right)),
        "ends_with" => actual
            .as_str()
            .zip(expected.as_str())
            .is_some_and(|(left, right)| left.ends_with(right)),
        "in" => expected
            .as_array()
            .is_some_and(|values| values.contains(actual)),
        _ => return Err(format!("unknown filter operator: {operator}")),
    })
}
