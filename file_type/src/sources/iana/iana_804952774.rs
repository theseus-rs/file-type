use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_804952774: FileType = FileType {
    file_format: &FileFormat {
        id: 804_952_774,
        source_type: SourceType::Iana,
        name: "sipc",
        extensions: &[],
        media_types: &["application/sipc"],
        signatures: &[],
        related_formats: &[],
    },
};
