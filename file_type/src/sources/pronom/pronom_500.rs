use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_500: FileType = FileType {
    file_format: &FileFormat {
        id: 500,
        source_type: SourceType::Pronom,
        name: "Lotus Notes Database",
        extensions: &["ns3", "nsf"],
        media_types: &["application/vnd.lotus-notes"],
        signatures: &[],
        related_formats: &[],
    },
};
