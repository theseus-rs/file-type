use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1046049563: FileType = FileType {
    file_format: &FileFormat {
        id: 1_046_049_563,
        source_type: SourceType::Iana,
        name: "vnd.3lightssoftware.imagescal",
        extensions: &[],
        media_types: &["application/vnd.3lightssoftware.imagescal"],
        signatures: &[],
        related_formats: &[],
    },
};
