use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3938668571: FileType = FileType {
    file_format: &FileFormat {
        id: 3_938_668_571,
        source_type: SourceType::Iana,
        name: "vnd.pcos",
        extensions: &[],
        media_types: &["application/vnd.pcos"],
        signatures: &[],
        related_formats: &[],
    },
};
