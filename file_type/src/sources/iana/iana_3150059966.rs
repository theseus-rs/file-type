use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3150059966: FileFormat = FileFormat {
    id: 3_150_059_966,
    source_type: SourceType::Iana,
    name: "gif",
    extensions: &[],
    media_types: &["image/gif"],
    internal_signatures: &[],
    related_formats: &[],
};
