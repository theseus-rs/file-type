use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_316075578: FileType = FileType {
    file_format: &FileFormat {
        id: 316_075_578,
        source_type: SourceType::Iana,
        name: "vnd.dbf",
        extensions: &[],
        media_types: &["application/vnd.dbf"],
        signatures: &[],
        related_formats: &[],
    },
};
