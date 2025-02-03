use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_238874535: FileFormat = FileFormat {
    id: 238_874_535,
    source_type: SourceType::Linguist,
    name: "MiniZinc",
    extensions: &["mzn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
