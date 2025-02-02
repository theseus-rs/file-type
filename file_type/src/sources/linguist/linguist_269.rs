use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_269: FileFormat = FileFormat {
    id: 269,
    source_type: SourceType::Linguist,
    name: "Oxygene",
    extensions: &["oxygene"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
