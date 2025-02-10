use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_673108048: FileType = FileType {
    file_format: &FileFormat {
        id: 673_108_048,
        source_type: SourceType::Iana,
        name: "cdmi-domain",
        extensions: &[],
        media_types: &["application/cdmi-domain"],
        signatures: &[],
        related_formats: &[],
    },
};
