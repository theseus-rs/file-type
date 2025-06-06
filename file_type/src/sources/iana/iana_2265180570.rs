use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2265180570: FileType = FileType {
    file_format: &FileFormat {
        id: 2_265_180_570,
        source_type: SourceType::Iana,
        name: "vnd.wap.wmlc",
        extensions: &[],
        media_types: &["application/vnd.wap.wmlc"],
        signatures: &[],
        related_formats: &[],
    },
};
