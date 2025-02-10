use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_861307175: FileType = FileType {
    file_format: &FileFormat {
        id: 861_307_175,
        source_type: SourceType::Iana,
        name: "alto-networkmapfilter+json",
        extensions: &[],
        media_types: &["application/alto-networkmapfilter+json"],
        signatures: &[],
        related_formats: &[],
    },
};
