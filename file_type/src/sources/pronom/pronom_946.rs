use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_946: FileType = FileType {
    file_format: &FileFormat {
        id: 946,
        source_type: SourceType::Pronom,
        name: "Microsoft FrontPage",
        extensions: &["lck"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
