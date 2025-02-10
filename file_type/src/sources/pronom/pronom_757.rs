use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_757: FileType = FileType {
    file_format: &FileFormat {
        id: 757,
        source_type: SourceType::Pronom,
        name: "StarOffice Writer",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
