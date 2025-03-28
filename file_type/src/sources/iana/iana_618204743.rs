use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_618204743: FileType = FileType {
    file_format: &FileFormat {
        id: 618_204_743,
        source_type: SourceType::Iana,
        name: "EVRCWB0",
        extensions: &[],
        media_types: &["audio/EVRCWB0"],
        signatures: &[],
        related_formats: &[],
    },
};
