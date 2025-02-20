use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1316700297: FileType = FileType {
    file_format: &FileFormat {
        id: 1_316_700_297,
        source_type: SourceType::Iana,
        name: "wmf",
        extensions: &[],
        media_types: &["image/wmf"],
        signatures: &[],
        related_formats: &[],
    },
};
