use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_677210597: FileType = FileType {
    file_format: &FileFormat {
        id: 677_210_597,
        source_type: SourceType::Linguist,
        name: "Oberon",
        extensions: &["ob2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
