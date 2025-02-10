use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3259195021: FileType = FileType {
    file_format: &FileFormat {
        id: 3_259_195_021,
        source_type: SourceType::Iana,
        name: "vnd.dna",
        extensions: &[],
        media_types: &["application/vnd.dna"],
        signatures: &[],
        related_formats: &[],
    },
};
