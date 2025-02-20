use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1802856897: FileType = FileType {
    file_format: &FileFormat {
        id: 1_802_856_897,
        source_type: SourceType::Iana,
        name: "vnd.hcl-bireports",
        extensions: &[],
        media_types: &["application/vnd.hcl-bireports"],
        signatures: &[],
        related_formats: &[],
    },
};
