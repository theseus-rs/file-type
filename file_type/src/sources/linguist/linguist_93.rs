use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_93: FileFormat = FileFormat {
    id: 93,
    source_type: SourceType::Linguist,
    name: "ECL",
    extensions: &["ecl", "eclxml"],
    media_types: &["text/x-ecl"],
    signatures: &[],
    related_formats: &[],
};
