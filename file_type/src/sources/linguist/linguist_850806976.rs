use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_850806976: FileType = FileType {
    file_format: &FileFormat {
        id: 850_806_976,
        source_type: SourceType::Linguist,
        name: "Cypher",
        extensions: &["cyp", "cypher"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
