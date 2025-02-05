use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3938229820: FileFormat = FileFormat {
    id: 3_938_229_820,
    source_type: SourceType::Iana,
    name: "vnd.sus-calendar",
    extensions: &[],
    media_types: &["application/vnd.sus-calendar"],
    signatures: &[],
    related_formats: &[],
};
