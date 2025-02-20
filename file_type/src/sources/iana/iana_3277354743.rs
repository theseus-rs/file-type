use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3277354743: FileType = FileType {
    file_format: &FileFormat {
        id: 3_277_354_743,
        source_type: SourceType::Iana,
        name: "vnd.apple.keynote",
        extensions: &[],
        media_types: &["application/vnd.apple.keynote"],
        signatures: &[],
        related_formats: &[],
    },
};
