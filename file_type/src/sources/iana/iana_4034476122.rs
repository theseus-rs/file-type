use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4034476122: FileFormat = FileFormat {
    id: 4_034_476_122,
    source_type: SourceType::Iana,
    name: "vnd.apple.pages",
    extensions: &[],
    media_types: &["application/vnd.apple.pages"],
    internal_signatures: &[],
    related_formats: &[],
};
