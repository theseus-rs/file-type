use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_501697374: FileType = FileType {
    file_format: &FileFormat {
        id: 501_697_374,
        source_type: SourceType::Iana,
        name: "dca-rft",
        extensions: &[],
        media_types: &["application/dca-rft"],
        signatures: &[],
        related_formats: &[],
    },
};
