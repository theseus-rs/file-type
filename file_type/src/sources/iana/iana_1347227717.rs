use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1347227717: FileType = FileType {
    file_format: &FileFormat {
        id: 1_347_227_717,
        source_type: SourceType::Iana,
        name: "vnd.oma.bcast.associated-procedure-parameter+xml",
        extensions: &[],
        media_types: &["application/vnd.oma.bcast.associated-procedure-parameter+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
