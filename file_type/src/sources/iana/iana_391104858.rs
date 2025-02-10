use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_391104858: FileType = FileType {
    file_format: &FileFormat {
        id: 391_104_858,
        source_type: SourceType::Iana,
        name: "reginfo+xml",
        extensions: &[],
        media_types: &["application/reginfo+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
