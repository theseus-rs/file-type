use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2038814550: FileType = FileType {
    file_format: &FileFormat {
        id: 2_038_814_550,
        source_type: SourceType::Iana,
        name: "1d-interleaved-parityfec",
        extensions: &[],
        media_types: &["video/1d-interleaved-parityfec"],
        signatures: &[],
        related_formats: &[],
    },
};
