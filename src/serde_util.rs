use serde::{Serialize, Serializer};

pub(crate) fn comma_separated<S, T>(
    value: &Option<Vec<T>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    let Some(items) = value else {
        return serializer.serialize_none();
    };

    if items.is_empty() {
        return serializer.serialize_none();
    };

    let joined = items
        .iter()
        .map(|item| {
            serde_json::to_value(item)
                .ok()
                .and_then(|v| v.as_str().map(str::to_owned))
                .expect("query enum should serialize to a plain string")
        })
        .collect::<Vec<_>>()
        .join(",");
    serializer.serialize_some(&joined)
}
