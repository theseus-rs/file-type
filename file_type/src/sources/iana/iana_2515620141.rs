use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2515620141: FileType = FileType {
    file_format: &FileFormat {
        id: 2_515_620_141,
        source_type: SourceType::Iana,
        name: "x3d+fastinfoset",
        extensions: &[],
        media_types: &["model/x3d+fastinfoset"],
        signatures: &[],
        related_formats: &[],
    },
};
