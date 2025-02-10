use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_863547179: FileType = FileType {
    file_format: &FileFormat {
        id: 863_547_179,
        source_type: SourceType::Iana,
        name: "xml-dtd",
        extensions: &[],
        media_types: &["application/xml-dtd"],
        signatures: &[],
        related_formats: &[],
    },
};
