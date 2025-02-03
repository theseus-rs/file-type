use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2159550589: FileFormat = FileFormat {
    id: 2_159_550_589,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-officedocument.presentationml.tableStyles+xml",
    extensions: &[],
    media_types: &["application/vnd.openxmlformats-officedocument.presentationml.tableStyles+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
