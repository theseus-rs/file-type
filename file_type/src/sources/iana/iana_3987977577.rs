use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3987977577: FileType = FileType {
    file_format: &FileFormat {
        id: 3_987_977_577,
        source_type: SourceType::Iana,
        name: "vnd.bint.med-content",
        extensions: &[],
        media_types: &["application/vnd.bint.med-content"],
        signatures: &[],
        related_formats: &[],
    },
};
