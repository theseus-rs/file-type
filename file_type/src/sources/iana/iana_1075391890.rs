use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1075391890: FileType = FileType {
    file_format: &FileFormat {
        id: 1_075_391_890,
        source_type: SourceType::Iana,
        name: "vnd.ezpix-album",
        extensions: &[],
        media_types: &["application/vnd.ezpix-album"],
        signatures: &[],
        related_formats: &[],
    },
};
