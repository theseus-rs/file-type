use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_319002153: FileFormat = FileFormat {
    id: 319_002_153,
    source_type: SourceType::Linguist,
    name: "ReasonLIGO",
    extensions: &["religo"],
    media_types: &["text/x-rustsrc"],
    internal_signatures: &[],
    related_formats: &[],
};
