use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_94: FileFormat = FileFormat {
    id: 94,
    source_type: SourceType::Linguist,
    name: "ECLiPSe",
    extensions: &["ecl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
