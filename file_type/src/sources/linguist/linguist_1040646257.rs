use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_1040646257: FileFormat = FileFormat {
    id: 1_040_646_257,
    source_type: SourceType::Linguist,
    name: "LigoLANG",
    extensions: &["ligo"],
    media_types: &["text/x-pascal"],
    internal_signatures: &[],
    related_formats: &[],
};
