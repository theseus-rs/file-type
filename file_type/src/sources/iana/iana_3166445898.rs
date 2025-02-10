use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3166445898: FileType = FileType {
    file_format: &FileFormat {
        id: 3_166_445_898,
        source_type: SourceType::Iana,
        name: "mesh",
        extensions: &[],
        media_types: &["model/mesh"],
        signatures: &[],
        related_formats: &[],
    },
};
