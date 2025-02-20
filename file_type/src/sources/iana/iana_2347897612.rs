use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2347897612: FileType = FileType {
    file_format: &FileFormat {
        id: 2_347_897_612,
        source_type: SourceType::Iana,
        name: "MP2T",
        extensions: &[],
        media_types: &["video/MP2T"],
        signatures: &[],
        related_formats: &[],
    },
};
