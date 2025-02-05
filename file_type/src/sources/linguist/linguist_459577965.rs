use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_459577965: FileFormat = FileFormat {
    id: 459_577_965,
    source_type: SourceType::Linguist,
    name: "GEDCOM",
    extensions: &["ged"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
