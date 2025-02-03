use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3386887635: FileFormat = FileFormat {
    id: 3_386_887_635,
    source_type: SourceType::Iana,
    name: "vnd.obn",
    extensions: &[],
    media_types: &["application/vnd.obn"],
    internal_signatures: &[],
    related_formats: &[],
};
