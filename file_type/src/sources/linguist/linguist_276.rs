use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_276: FileType = FileType {
    file_format: &FileFormat {
        id: 276,
        source_type: SourceType::Linguist,
        name: "Pan",
        extensions: &["pan"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
