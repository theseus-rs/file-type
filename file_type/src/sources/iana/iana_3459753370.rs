use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3459753370: FileType = FileType {
    file_format: &FileFormat {
        id: 3_459_753_370,
        source_type: SourceType::Iana,
        name: "smil (OBSOLETED in favor of application/smil+xml)",
        extensions: &[],
        media_types: &["application/smil"],
        signatures: &[],
        related_formats: &[],
    },
};
