use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2233227456: FileType = FileType {
    file_format: &FileFormat {
        id: 2_233_227_456,
        source_type: SourceType::Iana,
        name: "vnd.fdsn.mseed",
        extensions: &[],
        media_types: &["application/vnd.fdsn.mseed"],
        signatures: &[],
        related_formats: &[],
    },
};
