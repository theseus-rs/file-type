use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3086100026: FileType = FileType {
    file_format: &FileFormat {
        id: 3_086_100_026,
        source_type: SourceType::Iana,
        name: "vnd.oxli.countgraph",
        extensions: &[],
        media_types: &["application/vnd.oxli.countgraph"],
        signatures: &[],
        related_formats: &[],
    },
};
