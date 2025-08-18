use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_414: FileType = FileType {
    file_format: &FileFormat {
        id: 414,
        source_type: SourceType::Linguist,
        name: "edn",
        extensions: &["edn"],
        media_types: &["application/edn"],
        signatures: &[],
        related_formats: &[],
    },
};
