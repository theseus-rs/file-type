use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1929696087: FileType = FileType {
    file_format: &FileFormat {
        id: 1_929_696_087,
        source_type: SourceType::Iana,
        name: "vnd.uri-map",
        extensions: &[],
        media_types: &["application/vnd.uri-map"],
        signatures: &[],
        related_formats: &[],
    },
};
