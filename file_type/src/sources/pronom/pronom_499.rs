use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_499: FileType = FileType {
    file_format: &FileFormat {
        id: 499,
        source_type: SourceType::Pronom,
        name: "Lotus Notes Database",
        extensions: &["ns2", "nsf"],
        media_types: &["application/vnd.lotus-notes"],
        signatures: &[],
        related_formats: &[],
    },
};
