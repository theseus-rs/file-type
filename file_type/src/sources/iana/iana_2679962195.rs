use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2679962195: FileType = FileType {
    file_format: &FileFormat {
        id: 2_679_962_195,
        source_type: SourceType::Iana,
        name: "vnd.amundsen.maze+xml",
        extensions: &[],
        media_types: &["application/vnd.amundsen.maze+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
