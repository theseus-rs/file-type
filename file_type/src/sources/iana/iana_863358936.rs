use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_863358936: FileType = FileType {
    file_format: &FileFormat {
        id: 863_358_936,
        source_type: SourceType::Iana,
        name: "cnrp+xml",
        extensions: &[],
        media_types: &["application/cnrp+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
