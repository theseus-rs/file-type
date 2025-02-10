use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4001673838: FileType = FileType {
    file_format: &FileFormat {
        id: 4_001_673_838,
        source_type: SourceType::Iana,
        name: "vnd.efi.iso",
        extensions: &[],
        media_types: &["application/vnd.efi.iso"],
        signatures: &[],
        related_formats: &[],
    },
};
