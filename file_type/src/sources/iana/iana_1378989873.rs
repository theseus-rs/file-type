use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1378989873: FileType = FileType {
    file_format: &FileFormat {
        id: 1_378_989_873,
        source_type: SourceType::Iana,
        name: "pvd+json",
        extensions: &[],
        media_types: &["application/pvd+json"],
        signatures: &[],
        related_formats: &[],
    },
};
