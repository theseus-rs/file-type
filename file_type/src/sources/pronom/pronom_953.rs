use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_953: FileType = FileType {
    file_format: &FileFormat {
        id: 953,
        source_type: SourceType::Pronom,
        name: "Microsoft Works Database for Windows",
        extensions: &["wdb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
