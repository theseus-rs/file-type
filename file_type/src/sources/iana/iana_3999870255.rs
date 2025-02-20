use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3999870255: FileType = FileType {
    file_format: &FileFormat {
        id: 3_999_870_255,
        source_type: SourceType::Iana,
        name: "vnd.mophun.application",
        extensions: &[],
        media_types: &["application/vnd.mophun.application"],
        signatures: &[],
        related_formats: &[],
    },
};
