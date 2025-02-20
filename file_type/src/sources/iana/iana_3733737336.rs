use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3733737336: FileType = FileType {
    file_format: &FileFormat {
        id: 3_733_737_336,
        source_type: SourceType::Iana,
        name: "H263-2000",
        extensions: &[],
        media_types: &["video/H263-2000"],
        signatures: &[],
        related_formats: &[],
    },
};
