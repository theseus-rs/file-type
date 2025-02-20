use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_1612: FileType = FileType {
    file_format: &FileFormat {
        id: 1_612,
        source_type: SourceType::Pronom,
        name: "StarOffice Writer",
        extensions: &["sdw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
