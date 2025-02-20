use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_482269533: FileType = FileType {
    file_format: &FileFormat {
        id: 482_269_533,
        source_type: SourceType::Iana,
        name: "vnd.uoml+xml",
        extensions: &[],
        media_types: &["application/vnd.uoml+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
