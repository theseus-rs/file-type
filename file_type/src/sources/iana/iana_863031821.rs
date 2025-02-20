use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_863031821: FileType = FileType {
    file_format: &FileFormat {
        id: 863_031_821,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.gtpc",
        extensions: &[],
        media_types: &["application/vnd.3gpp.gtpc"],
        signatures: &[],
        related_formats: &[],
    },
};
