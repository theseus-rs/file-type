use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_363378884: FileFormat = FileFormat {
    id: 363_378_884,
    source_type: SourceType::Linguist,
    name: "Regular Expression",
    extensions: &["regex", "regexp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
