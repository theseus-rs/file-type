use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4103675381: FileType = FileType {
    file_format: &FileFormat {
        id: 4_103_675_381,
        source_type: SourceType::Iana,
        name: "vnd.epson.salt",
        extensions: &[],
        media_types: &["application/vnd.epson.salt"],
        signatures: &[],
        related_formats: &[],
    },
};
