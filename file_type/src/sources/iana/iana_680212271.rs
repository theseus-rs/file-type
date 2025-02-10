use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_680212271: FileType = FileType {
    file_format: &FileFormat {
        id: 680_212_271,
        source_type: SourceType::Iana,
        name: "vnd.fujixerox.edmics-rlc",
        extensions: &[],
        media_types: &["image/vnd.fujixerox.edmics-rlc"],
        signatures: &[],
        related_formats: &[],
    },
};
