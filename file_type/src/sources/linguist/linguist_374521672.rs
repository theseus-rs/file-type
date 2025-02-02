use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_374521672: FileFormat = FileFormat {
    id: 374_521_672,
    source_type: SourceType::Linguist,
    name: "WDL",
    extensions: &["wdl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
