use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1084097919: FileType = FileType {
    file_format: &FileFormat {
        id: 1_084_097_919,
        source_type: SourceType::Iana,
        name: "vnd.wrq-hp3000-labelled",
        extensions: &[],
        media_types: &["application/vnd.wrq-hp3000-labelled"],
        signatures: &[],
        related_formats: &[],
    },
};
