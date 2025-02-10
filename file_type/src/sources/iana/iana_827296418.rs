use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_827296418: FileType = FileType {
    file_format: &FileFormat {
        id: 827_296_418,
        source_type: SourceType::Iana,
        name: "jxl",
        extensions: &[],
        media_types: &["image/jxl"],
        signatures: &[],
        related_formats: &[],
    },
};
