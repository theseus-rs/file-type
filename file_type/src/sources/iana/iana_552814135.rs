use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_552814135: FileType = FileType {
    file_format: &FileFormat {
        id: 552_814_135,
        source_type: SourceType::Iana,
        name: "vnd.dataresource+json",
        extensions: &[],
        media_types: &["application/vnd.dataresource+json"],
        signatures: &[],
        related_formats: &[],
    },
};
