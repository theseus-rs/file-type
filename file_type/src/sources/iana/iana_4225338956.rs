use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4225338956: FileType = FileType {
    file_format: &FileFormat {
        id: 4_225_338_956,
        source_type: SourceType::Iana,
        name: "vnd.dxf",
        extensions: &[],
        media_types: &["image/vnd.dxf"],
        signatures: &[],
        related_formats: &[],
    },
};
