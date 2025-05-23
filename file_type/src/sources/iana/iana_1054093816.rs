use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1054093816: FileType = FileType {
    file_format: &FileFormat {
        id: 1_054_093_816,
        source_type: SourceType::Iana,
        name: "vnd.oma.bcast.simple-symbol-container",
        extensions: &[],
        media_types: &["application/vnd.oma.bcast.simple-symbol-container"],
        signatures: &[],
        related_formats: &[],
    },
};
