use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1798117158: FileType = FileType {
    file_format: &FileFormat {
        id: 1_798_117_158,
        source_type: SourceType::Iana,
        name: "vnd.powerbuilder75",
        extensions: &[],
        media_types: &["application/vnd.powerbuilder75"],
        signatures: &[],
        related_formats: &[],
    },
};
