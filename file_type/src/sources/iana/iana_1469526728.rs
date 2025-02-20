use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1469526728: FileType = FileType {
    file_format: &FileFormat {
        id: 1_469_526_728,
        source_type: SourceType::Iana,
        name: "jp2",
        extensions: &[],
        media_types: &["image/jp2"],
        signatures: &[],
        related_formats: &[],
    },
};
