use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2272512056: FileType = FileType {
    file_format: &FileFormat {
        id: 2_272_512_056,
        source_type: SourceType::Iana,
        name: "hyperstudio",
        extensions: &[],
        media_types: &["application/hyperstudio"],
        signatures: &[],
        related_formats: &[],
    },
};
