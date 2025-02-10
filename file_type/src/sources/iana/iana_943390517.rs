use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_943390517: FileType = FileType {
    file_format: &FileFormat {
        id: 943_390_517,
        source_type: SourceType::Iana,
        name: "vnd.oma.bcast.ltkm",
        extensions: &[],
        media_types: &["application/vnd.oma.bcast.ltkm"],
        signatures: &[],
        related_formats: &[],
    },
};
