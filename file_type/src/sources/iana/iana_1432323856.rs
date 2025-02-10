use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1432323856: FileType = FileType {
    file_format: &FileFormat {
        id: 1_432_323_856,
        source_type: SourceType::Iana,
        name: "vnd.paos.xml",
        extensions: &[],
        media_types: &["application/vnd.paos.xml"],
        signatures: &[],
        related_formats: &[],
    },
};
