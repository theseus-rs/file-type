use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2288395329: FileFormat = FileFormat {
    id: 2_288_395_329,
    source_type: SourceType::Iana,
    name: "vnd.curl",
    extensions: &[],
    media_types: &["text/vnd.curl"],
    signatures: &[],
    related_formats: &[],
};
