use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_404372107: FileType = FileType {
    file_format: &FileFormat {
        id: 404_372_107,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.mcdata-ue-config+xml",
        extensions: &[],
        media_types: &["application/vnd.3gpp.mcdata-ue-config+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
