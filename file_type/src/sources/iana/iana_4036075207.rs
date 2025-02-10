use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4036075207: FileType = FileType {
    file_format: &FileFormat {
        id: 4_036_075_207,
        source_type: SourceType::Iana,
        name: "vnd.vsf",
        extensions: &[],
        media_types: &["application/vnd.vsf"],
        signatures: &[],
        related_formats: &[],
    },
};
