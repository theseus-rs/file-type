use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3302833616: FileFormat = FileFormat {
    id: 3_302_833_616,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-officedocument.spreadsheetml.pivotTable+xml",
    extensions: &[],
    media_types: &["application/vnd.openxmlformats-officedocument.spreadsheetml.pivotTable+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
