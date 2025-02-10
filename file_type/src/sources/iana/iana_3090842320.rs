use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3090842320: FileType = FileType {
    file_format: &FileFormat {
        id: 3_090_842_320,
        source_type: SourceType::Iana,
        name: "step+zip",
        extensions: &[],
        media_types: &["model/step+zip"],
        signatures: &[],
        related_formats: &[],
    },
};
