use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_374945877: FileType = FileType {
    file_format: &FileFormat {
        id: 374_945_877,
        source_type: SourceType::Iana,
        name: "xcap-el+xml",
        extensions: &[],
        media_types: &["application/xcap-el+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
