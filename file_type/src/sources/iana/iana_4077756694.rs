use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4077756694: FileType = FileType {
    file_format: &FileFormat {
        id: 4_077_756_694,
        source_type: SourceType::Iana,
        name: "vnd.valve.source.texture",
        extensions: &[],
        media_types: &["image/vnd.valve.source.texture"],
        signatures: &[],
        related_formats: &[],
    },
};
