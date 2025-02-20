use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3932124695: FileType = FileType {
    file_format: &FileFormat {
        id: 3_932_124_695,
        source_type: SourceType::Iana,
        name: "vnd.dzr",
        extensions: &[],
        media_types: &["application/vnd.dzr"],
        signatures: &[],
        related_formats: &[],
    },
};
