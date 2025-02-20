use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3338420964: FileType = FileType {
    file_format: &FileFormat {
        id: 3_338_420_964,
        source_type: SourceType::Iana,
        name: "vnd.maxmind.maxmind-db",
        extensions: &[],
        media_types: &["application/vnd.maxmind.maxmind-db"],
        signatures: &[],
        related_formats: &[],
    },
};
