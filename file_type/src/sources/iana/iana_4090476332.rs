use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4090476332: FileType = FileType {
    file_format: &FileFormat {
        id: 4_090_476_332,
        source_type: SourceType::Iana,
        name: "vnd.oasis.opendocument.database (OBSOLETED in favor of application/vnd.oasis.opendocument.base)",
        extensions: &[],
        media_types: &["application/vnd.oasis.opendocument.database"],
        signatures: &[],
        related_formats: &[],
    },
};
