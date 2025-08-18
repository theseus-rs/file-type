use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3277566946: FileType = FileType {
    file_format: &FileFormat {
        id: 3_277_566_946,
        source_type: SourceType::Iana,
        name: "vnd.pp.systemverify+xml",
        extensions: &[],
        media_types: &["application/vnd.pp.systemverify+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
