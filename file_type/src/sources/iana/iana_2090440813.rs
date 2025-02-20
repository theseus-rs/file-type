use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2090440813: FileType = FileType {
    file_format: &FileFormat {
        id: 2_090_440_813,
        source_type: SourceType::Iana,
        name: "whoispp-response",
        extensions: &[],
        media_types: &["application/whoispp-response"],
        signatures: &[],
        related_formats: &[],
    },
};
