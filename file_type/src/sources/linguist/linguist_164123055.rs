use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_164123055: FileType = FileType {
    file_format: &FileFormat {
        id: 164_123_055,
        source_type: SourceType::Linguist,
        name: "SmPL",
        extensions: &["cocci"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
