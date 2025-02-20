use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1651655840: FileType = FileType {
    file_format: &FileFormat {
        id: 1_651_655_840,
        source_type: SourceType::Iana,
        name: "vc1",
        extensions: &[],
        media_types: &["video/vc1"],
        signatures: &[],
        related_formats: &[],
    },
};
