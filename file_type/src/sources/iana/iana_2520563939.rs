use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2520563939: FileType = FileType {
    file_format: &FileFormat {
        id: 2_520_563_939,
        source_type: SourceType::Iana,
        name: "vnd.gridmp",
        extensions: &[],
        media_types: &["application/vnd.gridmp"],
        signatures: &[],
        related_formats: &[],
    },
};
