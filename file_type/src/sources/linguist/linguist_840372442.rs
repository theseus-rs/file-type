use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_840372442: FileFormat = FileFormat {
    id: 840_372_442,
    source_type: SourceType::Linguist,
    name: "Pep8",
    extensions: &["pep"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
