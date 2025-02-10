use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1724910760: FileType = FileType {
    file_format: &FileFormat {
        id: 1_724_910_760,
        source_type: SourceType::Iana,
        name: "vnd.powerbuilder75-s",
        extensions: &[],
        media_types: &["application/vnd.powerbuilder75-s"],
        signatures: &[],
        related_formats: &[],
    },
};
