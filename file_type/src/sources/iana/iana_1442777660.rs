use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1442777660: FileFormat = FileFormat {
    id: 1_442_777_660,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml",
    extensions: &[],
    media_types: &["application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
