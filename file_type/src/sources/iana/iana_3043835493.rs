use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3043835493: FileType = FileType {
    file_format: &FileFormat {
        id: 3_043_835_493,
        source_type: SourceType::Iana,
        name: "MP2P",
        extensions: &[],
        media_types: &["video/MP2P"],
        signatures: &[],
        related_formats: &[],
    },
};
