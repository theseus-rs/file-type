use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2167279598: FileType = FileType {
    file_format: &FileFormat {
        id: 2_167_279_598,
        source_type: SourceType::Iana,
        name: "vnd.veritone.aion+json",
        extensions: &[],
        media_types: &["application/vnd.veritone.aion+json"],
        signatures: &[],
        related_formats: &[],
    },
};
