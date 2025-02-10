use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1847799466: FileType = FileType {
    file_format: &FileFormat {
        id: 1_847_799_466,
        source_type: SourceType::Iana,
        name: "vnd.ims.imsccv1p3",
        extensions: &[],
        media_types: &["application/vnd.ims.imsccv1p3"],
        signatures: &[],
        related_formats: &[],
    },
};
