use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2599515299: FileType = FileType {
    file_format: &FileFormat {
        id: 2_599_515_299,
        source_type: SourceType::Iana,
        name: "vnd.oma.bcast.sgboot",
        extensions: &[],
        media_types: &["application/vnd.oma.bcast.sgboot"],
        signatures: &[],
        related_formats: &[],
    },
};
