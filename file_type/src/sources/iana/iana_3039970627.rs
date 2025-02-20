use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3039970627: FileType = FileType {
    file_format: &FileFormat {
        id: 3_039_970_627,
        source_type: SourceType::Iana,
        name: "vnd.net-fpx",
        extensions: &[],
        media_types: &["image/vnd.net-fpx"],
        signatures: &[],
        related_formats: &[],
    },
};
