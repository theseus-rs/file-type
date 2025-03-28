use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3909547953: FileType = FileType {
    file_format: &FileFormat {
        id: 3_909_547_953,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.5gnas",
        extensions: &[],
        media_types: &["application/vnd.3gpp.5gnas"],
        signatures: &[],
        related_formats: &[],
    },
};
