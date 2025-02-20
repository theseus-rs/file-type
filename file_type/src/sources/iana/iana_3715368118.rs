use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3715368118: FileType = FileType {
    file_format: &FileFormat {
        id: 3_715_368_118,
        source_type: SourceType::Iana,
        name: "vnd.dlna.adts",
        extensions: &[],
        media_types: &["audio/vnd.dlna.adts"],
        signatures: &[],
        related_formats: &[],
    },
};
