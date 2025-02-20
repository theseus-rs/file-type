use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_388454346: FileType = FileType {
    file_format: &FileFormat {
        id: 388_454_346,
        source_type: SourceType::Iana,
        name: "vnd.ms-windows.wsd.oob",
        extensions: &[],
        media_types: &["application/vnd.ms-windows.wsd.oob"],
        signatures: &[],
        related_formats: &[],
    },
};
