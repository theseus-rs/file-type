use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_933758247: FileType = FileType {
    file_format: &FileFormat {
        id: 933_758_247,
        source_type: SourceType::Iana,
        name: "vc+cose",
        extensions: &[],
        media_types: &["application/vc+cose"],
        signatures: &[],
        related_formats: &[],
    },
};
