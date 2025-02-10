use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_359796587: FileType = FileType {
    file_format: &FileFormat {
        id: 359_796_587,
        source_type: SourceType::Iana,
        name: "vnd.noblenet-sealer",
        extensions: &[],
        media_types: &["application/vnd.noblenet-sealer"],
        signatures: &[],
        related_formats: &[],
    },
};
