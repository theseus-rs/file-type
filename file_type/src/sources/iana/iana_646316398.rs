use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_646316398: FileType = FileType {
    file_format: &FileFormat {
        id: 646_316_398,
        source_type: SourceType::Iana,
        name: "vnd.ffsns",
        extensions: &[],
        media_types: &["application/vnd.ffsns"],
        signatures: &[],
        related_formats: &[],
    },
};
