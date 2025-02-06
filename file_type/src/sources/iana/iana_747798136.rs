use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_747798136: FileFormat = FileFormat {
    id: 747_798_136,
    source_type: SourceType::Iana,
    name: "tzif",
    extensions: &[],
    media_types: &["application/tzif"],
    signatures: &[],
    related_formats: &[],
};
