use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1246631980: FileType = FileType {
    file_format: &FileFormat {
        id: 1_246_631_980,
        source_type: SourceType::Iana,
        name: "vnd.fdsn.seed",
        extensions: &[],
        media_types: &["application/vnd.fdsn.seed"],
        signatures: &[],
        related_formats: &[],
    },
};
