use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1223968204: FileType = FileType {
    file_format: &FileFormat {
        id: 1_223_968_204,
        source_type: SourceType::Iana,
        name: "telephone-event",
        extensions: &[],
        media_types: &["audio/telephone-event"],
        signatures: &[],
        related_formats: &[],
    },
};
