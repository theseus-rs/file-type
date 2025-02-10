use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_140: FileType = FileType {
    file_format: &FileFormat {
        id: 140,
        source_type: SourceType::Pronom,
        name: "Postscript Support File",
        extensions: &["psf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
