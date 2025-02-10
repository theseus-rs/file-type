use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1251073663: FileType = FileType {
    file_format: &FileFormat {
        id: 1_251_073_663,
        source_type: SourceType::Iana,
        name: "vnd.nitf",
        extensions: &[],
        media_types: &["application/vnd.nitf"],
        signatures: &[],
        related_formats: &[],
    },
};
