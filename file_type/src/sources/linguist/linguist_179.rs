use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_179: FileFormat = FileFormat {
    id: 179,
    source_type: SourceType::Linguist,
    name: "Pug",
    extensions: &["jade", "pug"],
    media_types: &["text/x-pug"],
    signatures: &[],
    related_formats: &[],
};
