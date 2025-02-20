use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_472182639: FileType = FileType {
    file_format: &FileFormat {
        id: 472_182_639,
        source_type: SourceType::Iana,
        name: "1d-interleaved-parityfec",
        extensions: &[],
        media_types: &["application/1d-interleaved-parityfec"],
        signatures: &[],
        related_formats: &[],
    },
};
