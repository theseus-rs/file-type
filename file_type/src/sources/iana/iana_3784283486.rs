use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3784283486: FileType = FileType {
    file_format: &FileFormat {
        id: 3_784_283_486,
        source_type: SourceType::Iana,
        name: "example",
        extensions: &[],
        media_types: &["audio/example"],
        signatures: &[],
        related_formats: &[],
    },
};
