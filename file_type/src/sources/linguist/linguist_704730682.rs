use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_704730682: FileFormat = FileFormat {
    id: 704_730_682,
    source_type: SourceType::Linguist,
    name: "Typst",
    extensions: &["typ"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
