use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2759718441: FileFormat = FileFormat {
    id: 2_759_718_441,
    source_type: SourceType::Iana,
    name: "tamp-community-update",
    extensions: &[],
    media_types: &["application/tamp-community-update"],
    internal_signatures: &[],
    related_formats: &[],
};
