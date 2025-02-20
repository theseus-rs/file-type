use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_362916929: FileType = FileType {
    file_format: &FileFormat {
        id: 362_916_929,
        source_type: SourceType::Iana,
        name: "vnd.syncml.dmddf+xml",
        extensions: &[],
        media_types: &["application/vnd.syncml.dmddf+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
