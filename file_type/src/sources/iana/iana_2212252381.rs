use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2212252381: FileType = FileType {
    file_format: &FileFormat {
        id: 2_212_252_381,
        source_type: SourceType::Iana,
        name: "vnd.etsi.iptvsad-cod+xml",
        extensions: &[],
        media_types: &["application/vnd.etsi.iptvsad-cod+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
