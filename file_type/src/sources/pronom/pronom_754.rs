use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_754: FileType = FileType {
    file_format: &FileFormat {
        id: 754,
        source_type: SourceType::Pronom,
        name: "StarOffice Writer",
        extensions: &["sdw"],
        media_types: &["application/vnd.stardivision.writer"],
        signatures: &[],
        related_formats: &[],
    },
};
