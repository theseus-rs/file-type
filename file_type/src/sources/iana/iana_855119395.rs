use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_855119395: FileType = FileType {
    file_format: &FileFormat {
        id: 855_119_395,
        source_type: SourceType::Iana,
        name: "vnd.uiq.theme",
        extensions: &[],
        media_types: &["application/vnd.uiq.theme"],
        signatures: &[],
        related_formats: &[],
    },
};
