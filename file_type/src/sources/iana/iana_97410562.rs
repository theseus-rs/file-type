use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_97410562: FileType = FileType {
    file_format: &FileFormat {
        id: 97_410_562,
        source_type: SourceType::Iana,
        name: "G719",
        extensions: &[],
        media_types: &["audio/G719"],
        signatures: &[],
        related_formats: &[],
    },
};
