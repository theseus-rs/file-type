use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1917324059: FileType = FileType {
    file_format: &FileFormat {
        id: 1_917_324_059,
        source_type: SourceType::Iana,
        name: "vnd.ah-barcode",
        extensions: &[],
        media_types: &["application/vnd.ah-barcode"],
        signatures: &[],
        related_formats: &[],
    },
};
