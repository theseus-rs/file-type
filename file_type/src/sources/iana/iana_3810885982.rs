use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3810885982: FileType = FileType {
    file_format: &FileFormat {
        id: 3_810_885_982,
        source_type: SourceType::Iana,
        name: "linkset+json",
        extensions: &[],
        media_types: &["application/linkset+json"],
        signatures: &[],
        related_formats: &[],
    },
};
