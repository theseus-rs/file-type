use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_36: FileType = FileType {
    file_format: &FileFormat {
        id: 36,
        source_type: SourceType::Linguist,
        name: "Bluespec",
        extensions: &["bsv"],
        media_types: &["text/x-systemverilog"],
        signatures: &[],
        related_formats: &[],
    },
};
