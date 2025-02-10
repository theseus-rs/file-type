use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_640881317: FileType = FileType {
    file_format: &FileFormat {
        id: 640_881_317,
        source_type: SourceType::Iana,
        name: "moss-signature",
        extensions: &[],
        media_types: &["application/moss-signature"],
        signatures: &[],
        related_formats: &[],
    },
};
