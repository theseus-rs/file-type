use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1673297055: FileType = FileType {
    file_format: &FileFormat {
        id: 1_673_297_055,
        source_type: SourceType::Iana,
        name: "tiff-fx",
        extensions: &[],
        media_types: &["image/tiff-fx"],
        signatures: &[],
        related_formats: &[],
    },
};
