use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3376783372: FileType = FileType {
    file_format: &FileFormat {
        id: 3_376_783_372,
        source_type: SourceType::Iana,
        name: "vnd.digital-winds",
        extensions: &[],
        media_types: &["audio/vnd.digital-winds"],
        signatures: &[],
        related_formats: &[],
    },
};
