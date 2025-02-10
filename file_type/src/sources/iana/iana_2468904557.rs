use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2468904557: FileType = FileType {
    file_format: &FileFormat {
        id: 2_468_904_557,
        source_type: SourceType::Iana,
        name: "vnd.radisys.moml+xml",
        extensions: &[],
        media_types: &["application/vnd.radisys.moml+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
