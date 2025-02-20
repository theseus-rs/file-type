use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_501: FileType = FileType {
    file_format: &FileFormat {
        id: 501,
        source_type: SourceType::Pronom,
        name: "Lotus Notes Database",
        extensions: &["ns4", "nsf"],
        media_types: &["application/vnd.lotus-notes"],
        signatures: &[],
        related_formats: &[],
    },
};
