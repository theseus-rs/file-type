use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3099405177: FileType = FileType {
    file_format: &FileFormat {
        id: 3_099_405_177,
        source_type: SourceType::Iana,
        name: "vnd.everad.plj",
        extensions: &[],
        media_types: &["audio/vnd.everad.plj"],
        signatures: &[],
        related_formats: &[],
    },
};
