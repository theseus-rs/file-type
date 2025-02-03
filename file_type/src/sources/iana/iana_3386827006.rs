use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3386827006: FileFormat = FileFormat {
    id: 3_386_827_006,
    source_type: SourceType::Iana,
    name: "ujcs+json",
    extensions: &[],
    media_types: &["application/ujcs+json"],
    internal_signatures: &[],
    related_formats: &[],
};
