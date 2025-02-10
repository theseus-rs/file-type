use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3192798696: FileType = FileType {
    file_format: &FileFormat {
        id: 3_192_798_696,
        source_type: SourceType::Iana,
        name: "vnd.msign",
        extensions: &[],
        media_types: &["application/vnd.msign"],
        signatures: &[],
        related_formats: &[],
    },
};
