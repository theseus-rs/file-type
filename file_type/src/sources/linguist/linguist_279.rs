use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_279: FileFormat = FileFormat {
    id: 279,
    source_type: SourceType::Linguist,
    name: "Parrot Assembly",
    extensions: &["pasm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
