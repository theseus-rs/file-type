use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2383623582: FileFormat = FileFormat {
    id: 2_383_623_582,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-officedocument.themeOverride+xml",
    extensions: &[],
    media_types: &["application/vnd.openxmlformats-officedocument.themeOverride+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
