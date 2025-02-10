use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3259195021: FileType = FileType {
    file_format: &FileFormat {
        id: 3_259_195_021,
        source_type: SourceType::Httpd,
        name: "dna",
        extensions: &["dna"],
        media_types: &["application/vnd.dna"],
        signatures: &[],
        related_formats: &[],
    },
};
