use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_310539211: FileType = FileType {
    file_format: &FileFormat {
        id: 310_539_211,
        source_type: SourceType::Iana,
        name: "ogg",
        extensions: &[],
        media_types: &["audio/ogg"],
        signatures: &[],
        related_formats: &[],
    },
};
