use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_276: FileFormat = FileFormat {
    id: 276,
    source_type: SourceType::Linguist,
    name: "Pan",
    extensions: &["pan"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
