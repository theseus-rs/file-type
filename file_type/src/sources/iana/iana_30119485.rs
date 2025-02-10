use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_30119485: FileType = FileType {
    file_format: &FileFormat {
        id: 30_119_485,
        source_type: SourceType::Iana,
        name: "vnd.tencent.tap",
        extensions: &[],
        media_types: &["image/vnd.tencent.tap"],
        signatures: &[],
        related_formats: &[],
    },
};
