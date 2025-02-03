use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_704715468: FileFormat = FileFormat {
    id: 704_715_468,
    source_type: SourceType::Iana,
    name: "G7221",
    extensions: &[],
    media_types: &["audio/G7221"],
    internal_signatures: &[],
    related_formats: &[],
};
