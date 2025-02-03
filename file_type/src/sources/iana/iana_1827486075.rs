use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1827486075: FileFormat = FileFormat {
    id: 1_827_486_075,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-officedocument.spreadsheetml.queryTable+xml",
    extensions: &[],
    media_types: &["application/vnd.openxmlformats-officedocument.spreadsheetml.queryTable+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
