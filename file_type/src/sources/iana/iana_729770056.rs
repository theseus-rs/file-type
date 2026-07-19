use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_729770056: FileType = FileType {
    file_format: &FileFormat {
        id: 729_770_056,
        source_type: SourceType::Iana,
        name: "vnd.blockfact.facts",
        extensions: &[],
        media_types: &["application/vnd.blockfact.facts"],
        signatures: &[],
        related_formats: &[],
    },
};
