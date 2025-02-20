use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3083309523: FileType = FileType {
    file_format: &FileFormat {
        id: 3_083_309_523,
        source_type: SourceType::Iana,
        name: "vnd.businessobjects",
        extensions: &[],
        media_types: &["application/vnd.businessobjects"],
        signatures: &[],
        related_formats: &[],
    },
};
