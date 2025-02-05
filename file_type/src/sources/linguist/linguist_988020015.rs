use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_988020015: FileFormat = FileFormat {
    id: 988_020_015,
    source_type: SourceType::Linguist,
    name: "Texinfo",
    extensions: &["texi", "texinfo", "txi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
