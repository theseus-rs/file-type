use crate::format::{FileFormat, SourceType};
use crate::FileType;

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
