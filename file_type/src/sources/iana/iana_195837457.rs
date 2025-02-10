use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_195837457: FileType = FileType {
    file_format: &FileFormat {
        id: 195_837_457,
        source_type: SourceType::Iana,
        name: "dls",
        extensions: &[],
        media_types: &["audio/dls"],
        signatures: &[],
        related_formats: &[],
    },
};
