use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4088672022: FileFormat = FileFormat {
    id: 4_088_672_022,
    source_type: SourceType::Iana,
    name: "vnd.cinderella",
    extensions: &[],
    media_types: &["application/vnd.cinderella"],
    internal_signatures: &[],
    related_formats: &[],
};
