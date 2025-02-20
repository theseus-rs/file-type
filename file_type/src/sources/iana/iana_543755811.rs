use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_543755811: FileType = FileType {
    file_format: &FileFormat {
        id: 543_755_811,
        source_type: SourceType::Iana,
        name: "vnd.etsi.timestamp-token",
        extensions: &[],
        media_types: &["application/vnd.etsi.timestamp-token"],
        signatures: &[],
        related_formats: &[],
    },
};
