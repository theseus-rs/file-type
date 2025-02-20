use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3434348798: FileType = FileType {
    file_format: &FileFormat {
        id: 3_434_348_798,
        source_type: SourceType::Iana,
        name: "vnd.eprints.data+xml",
        extensions: &[],
        media_types: &["application/vnd.eprints.data+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
