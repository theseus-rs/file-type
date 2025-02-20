use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_205455127: FileType = FileType {
    file_format: &FileFormat {
        id: 205_455_127,
        source_type: SourceType::Iana,
        name: "EVRC-QCP",
        extensions: &[],
        media_types: &["audio/EVRC-QCP"],
        signatures: &[],
        related_formats: &[],
    },
};
