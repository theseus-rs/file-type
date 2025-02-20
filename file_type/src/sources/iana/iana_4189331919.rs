use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4189331919: FileType = FileType {
    file_format: &FileFormat {
        id: 4_189_331_919,
        source_type: SourceType::Iana,
        name: "vnd.resilient.logic",
        extensions: &[],
        media_types: &["application/vnd.resilient.logic"],
        signatures: &[],
        related_formats: &[],
    },
};
