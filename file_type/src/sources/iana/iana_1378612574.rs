use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1378612574: FileType = FileType {
    file_format: &FileFormat {
        id: 1_378_612_574,
        source_type: SourceType::Iana,
        name: "vnd.ms-windows.printerpairing",
        extensions: &[],
        media_types: &["application/vnd.ms-windows.printerpairing"],
        signatures: &[],
        related_formats: &[],
    },
};
