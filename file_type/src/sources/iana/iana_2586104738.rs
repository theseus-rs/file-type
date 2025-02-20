use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2586104738: FileType = FileType {
    file_format: &FileFormat {
        id: 2_586_104_738,
        source_type: SourceType::Iana,
        name: "vnd.dece.ttml+xml",
        extensions: &[],
        media_types: &["application/vnd.dece.ttml+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
