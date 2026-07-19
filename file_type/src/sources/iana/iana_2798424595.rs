use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2798424595: FileType = FileType {
    file_format: &FileFormat {
        id: 2_798_424_595,
        source_type: SourceType::Iana,
        name: "vnd.xcdn",
        extensions: &[],
        media_types: &["application/vnd.xcdn"],
        signatures: &[],
        related_formats: &[],
    },
};
