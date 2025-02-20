use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_1804: FileType = FileType {
    file_format: &FileFormat {
        id: 1_804,
        source_type: SourceType::Pronom,
        name: "Krita Document Format",
        extensions: &["kra"],
        media_types: &["application/x-krita"],
        signatures: &[],
        related_formats: &[],
    },
};
