use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_476447814: FileType = FileType {
    file_format: &FileFormat {
        id: 476_447_814,
        source_type: SourceType::Linguist,
        name: "Cylc",
        extensions: &["cylc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
