use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3857626357: FileType = FileType {
    file_format: &FileFormat {
        id: 3_857_626_357,
        source_type: SourceType::Iana,
        name: "G729",
        extensions: &[],
        media_types: &["audio/G729"],
        signatures: &[],
        related_formats: &[],
    },
};
