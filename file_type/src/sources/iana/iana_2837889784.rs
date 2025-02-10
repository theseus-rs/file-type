use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2837889784: FileType = FileType {
    file_format: &FileFormat {
        id: 2_837_889_784,
        source_type: SourceType::Iana,
        name: "vnd.djvu",
        extensions: &[],
        media_types: &["image/vnd.djvu"],
        signatures: &[],
        related_formats: &[],
    },
};
