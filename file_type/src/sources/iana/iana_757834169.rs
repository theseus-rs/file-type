use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_757834169: FileType = FileType {
    file_format: &FileFormat {
        id: 757_834_169,
        source_type: SourceType::Iana,
        name: "vnd.ncd.reference",
        extensions: &[],
        media_types: &["application/vnd.ncd.reference"],
        signatures: &[],
        related_formats: &[],
    },
};
