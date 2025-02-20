use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3511919391: FileType = FileType {
    file_format: &FileFormat {
        id: 3_511_919_391,
        source_type: SourceType::Iana,
        name: "vnd.qcelp - DEPRECATED in favor of audio/qcelp",
        extensions: &[],
        media_types: &["audio/vnd.qcelp"],
        signatures: &[],
        related_formats: &[],
    },
};
