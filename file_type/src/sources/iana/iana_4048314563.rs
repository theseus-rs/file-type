use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4048314563: FileType = FileType {
    file_format: &FileFormat {
        id: 4_048_314_563,
        source_type: SourceType::Iana,
        name: "vnd.oipf.dae.xhtml+xml",
        extensions: &[],
        media_types: &["application/vnd.oipf.dae.xhtml+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
