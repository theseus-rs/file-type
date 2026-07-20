use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_864005057: FileType = FileType {
    file_format: &FileFormat {
        id: 864_005_057,
        source_type: SourceType::Linguist,
        name: "SpiceDB Schema",
        extensions: &["zed"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
