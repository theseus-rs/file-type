use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_390788699: FileFormat = FileFormat {
    id: 390_788_699,
    source_type: SourceType::Linguist,
    name: "CAP CDS",
    extensions: &["cds"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
