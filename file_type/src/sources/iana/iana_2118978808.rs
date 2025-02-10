use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2118978808: FileType = FileType {
    file_format: &FileFormat {
        id: 2_118_978_808,
        source_type: SourceType::Iana,
        name: "vnd.syncml.dm+xml",
        extensions: &[],
        media_types: &["application/vnd.syncml.dm+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
