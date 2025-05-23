use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_804: FileType = FileType {
    file_format: &FileFormat {
        id: 804,
        source_type: SourceType::Pronom,
        name: "HTML Extension File",
        extensions: &["htx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
