use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_116732393: FileType = FileType {
    file_format: &FileFormat {
        id: 116_732_393,
        source_type: SourceType::Iana,
        name: "alto-updatestreamparams+json",
        extensions: &[],
        media_types: &["application/alto-updatestreamparams+json"],
        signatures: &[],
        related_formats: &[],
    },
};
