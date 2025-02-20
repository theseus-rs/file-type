use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1229145838: FileType = FileType {
    file_format: &FileFormat {
        id: 1_229_145_838,
        source_type: SourceType::Iana,
        name: "vnd.amiga.ami",
        extensions: &[],
        media_types: &["application/vnd.amiga.ami"],
        signatures: &[],
        related_formats: &[],
    },
};
