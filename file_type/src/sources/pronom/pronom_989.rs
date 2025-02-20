use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_989: FileType = FileType {
    file_format: &FileFormat {
        id: 989,
        source_type: SourceType::Pronom,
        name: "Microsoft Works Word Processor Windows",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
