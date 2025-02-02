use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_111148035: FileFormat = FileFormat {
    id: 111_148_035,
    source_type: SourceType::Linguist,
    name: "Dotenv",
    extensions: &["env"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
