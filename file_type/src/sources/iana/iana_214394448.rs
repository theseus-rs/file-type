use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_214394448: FileType = FileType {
    file_format: &FileFormat {
        id: 214_394_448,
        source_type: SourceType::Iana,
        name: "vnd.wap.wmlscript",
        extensions: &[],
        media_types: &["text/vnd.wap.wmlscript"],
        signatures: &[],
        related_formats: &[],
    },
};
