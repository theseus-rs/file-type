use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_142: FileType = FileType {
    file_format: &FileFormat {
        id: 142,
        source_type: SourceType::Linguist,
        name: "Groovy",
        extensions: &["groovy", "grt", "gtpl", "gvy"],
        media_types: &["text/x-groovy"],
        signatures: &[],
        related_formats: &[],
    },
};
