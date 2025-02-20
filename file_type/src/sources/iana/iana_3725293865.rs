use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3725293865: FileType = FileType {
    file_format: &FileFormat {
        id: 3_725_293_865,
        source_type: SourceType::Iana,
        name: "EVRCB1",
        extensions: &[],
        media_types: &["audio/EVRCB1"],
        signatures: &[],
        related_formats: &[],
    },
};
