use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_381753813: FileType = FileType {
    file_format: &FileFormat {
        id: 381_753_813,
        source_type: SourceType::Iana,
        name: "mpeg4-generic",
        extensions: &[],
        media_types: &["application/mpeg4-generic"],
        signatures: &[],
        related_formats: &[],
    },
};
