use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
