use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_582798602: FileType = FileType {
    file_format: &FileFormat {
        id: 582_798_602,
        source_type: SourceType::Iana,
        name: "MELP2400",
        extensions: &[],
        media_types: &["audio/MELP2400"],
        signatures: &[],
        related_formats: &[],
    },
};
