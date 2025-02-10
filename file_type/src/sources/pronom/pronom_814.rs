use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_814: FileType = FileType {
    file_format: &FileFormat {
        id: 814,
        source_type: SourceType::Pronom,
        name: "Acrobat Language definition file",
        extensions: &["lng"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
