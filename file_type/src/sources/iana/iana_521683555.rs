use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_521683555: FileType = FileType {
    file_format: &FileFormat {
        id: 521_683_555,
        source_type: SourceType::Iana,
        name: "vnd.oma.bcast.provisioningtrigger",
        extensions: &[],
        media_types: &["application/vnd.oma.bcast.provisioningtrigger"],
        signatures: &[],
        related_formats: &[],
    },
};
