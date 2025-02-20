use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_400: FileType = FileType {
    file_format: &FileFormat {
        id: 400,
        source_type: SourceType::Linguist,
        name: "XPages",
        extensions: &["xsp-config", "xsp.metadata"],
        media_types: &["text/xml"],
        signatures: &[],
        related_formats: &[],
    },
};
