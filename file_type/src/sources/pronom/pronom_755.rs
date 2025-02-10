use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_755: FileType = FileType {
    file_format: &FileFormat {
        id: 755,
        source_type: SourceType::Pronom,
        name: "StarOffice Draw",
        extensions: &["sda"],
        media_types: &["application/vnd.stardivision.draw"],
        signatures: &[],
        related_formats: &[],
    },
};
