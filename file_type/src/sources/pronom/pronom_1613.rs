use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1613: FileType = FileType {
    file_format: &FileFormat {
        id: 1_613,
        source_type: SourceType::Pronom,
        name: "StarOffice Writer",
        extensions: &["sdw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
