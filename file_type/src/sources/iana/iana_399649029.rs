use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_399649029: FileType = FileType {
    file_format: &FileFormat {
        id: 399_649_029,
        source_type: SourceType::Iana,
        name: "vnd.oma.bcast.stkm",
        extensions: &[],
        media_types: &["application/vnd.oma.bcast.stkm"],
        signatures: &[],
        related_formats: &[],
    },
};
