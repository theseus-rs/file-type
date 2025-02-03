use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_432600901: FileFormat = FileFormat {
    id: 432_600_901,
    source_type: SourceType::Linguist,
    name: "Gradle Kotlin DSL",
    extensions: &["gradle.kts"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
