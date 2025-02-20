use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
