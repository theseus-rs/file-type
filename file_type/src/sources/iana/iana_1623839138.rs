use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1623839138: FileType = FileType {
    file_format: &FileFormat {
        id: 1_623_839_138,
        source_type: SourceType::Iana,
        name: "1d-interleaved-parityfec",
        extensions: &[],
        media_types: &["text/1d-interleaved-parityfec"],
        signatures: &[],
        related_formats: &[],
    },
};
