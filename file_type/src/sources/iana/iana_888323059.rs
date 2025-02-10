use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_888323059: FileType = FileType {
    file_format: &FileFormat {
        id: 888_323_059,
        source_type: SourceType::Iana,
        name: "vnd.seis+json",
        extensions: &[],
        media_types: &["application/vnd.seis+json"],
        signatures: &[],
        related_formats: &[],
    },
};
