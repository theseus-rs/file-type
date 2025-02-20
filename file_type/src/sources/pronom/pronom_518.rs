use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_518: FileType = FileType {
    file_format: &FileFormat {
        id: 518,
        source_type: SourceType::Pronom,
        name: "Professional Write Text File",
        extensions: &["pw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
