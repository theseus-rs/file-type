use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_510261994: FileType = FileType {
    file_format: &FileFormat {
        id: 510_261_994,
        source_type: SourceType::Iana,
        name: "G728",
        extensions: &[],
        media_types: &["audio/G728"],
        signatures: &[],
        related_formats: &[],
    },
};
