use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1968757943: FileType = FileType {
    file_format: &FileFormat {
        id: 1_968_757_943,
        source_type: SourceType::Iana,
        name: "vnd.oma.bcast.sgdu",
        extensions: &[],
        media_types: &["application/vnd.oma.bcast.sgdu"],
        signatures: &[],
        related_formats: &[],
    },
};
