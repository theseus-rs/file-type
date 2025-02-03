use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3482003446: FileFormat = FileFormat {
    id: 3_482_003_446,
    source_type: SourceType::Iana,
    name: "atomicmail",
    extensions: &[],
    media_types: &["application/atomicmail"],
    internal_signatures: &[],
    related_formats: &[],
};
