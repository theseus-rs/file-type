use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3585526041: FileType = FileType {
    file_format: &FileFormat {
        id: 3_585_526_041,
        source_type: SourceType::Iana,
        name: "scitt-statement+cose",
        extensions: &[],
        media_types: &["application/scitt-statement+cose"],
        signatures: &[],
        related_formats: &[],
    },
};
