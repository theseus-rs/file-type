use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2190611199: FileType = FileType {
    file_format: &FileFormat {
        id: 2_190_611_199,
        source_type: SourceType::Iana,
        name: "vnd.intu.qbo",
        extensions: &[],
        media_types: &["application/vnd.intu.qbo"],
        signatures: &[],
        related_formats: &[],
    },
};
