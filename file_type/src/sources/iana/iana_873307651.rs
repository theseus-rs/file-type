use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_873307651: FileType = FileType {
    file_format: &FileFormat {
        id: 873_307_651,
        source_type: SourceType::Iana,
        name: "vnd.pmtiles",
        extensions: &[],
        media_types: &["application/vnd.pmtiles"],
        signatures: &[],
        related_formats: &[],
    },
};
