use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3000218838: FileType = FileType {
    file_format: &FileFormat {
        id: 3_000_218_838,
        source_type: SourceType::Iana,
        name: "city+json-seq",
        extensions: &[],
        media_types: &["application/city+json-seq"],
        signatures: &[],
        related_formats: &[],
    },
};
