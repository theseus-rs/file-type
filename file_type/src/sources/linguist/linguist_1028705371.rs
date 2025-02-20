use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_1028705371: FileType = FileType {
    file_format: &FileFormat {
        id: 1_028_705_371,
        source_type: SourceType::Linguist,
        name: "Janet",
        extensions: &["janet"],
        media_types: &["text/x-scheme"],
        signatures: &[],
        related_formats: &[],
    },
};
