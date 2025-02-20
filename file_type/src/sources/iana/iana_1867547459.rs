use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1867547459: FileType = FileType {
    file_format: &FileFormat {
        id: 1_867_547_459,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.mcdata-signalling",
        extensions: &[],
        media_types: &["application/vnd.3gpp.mcdata-signalling"],
        signatures: &[],
        related_formats: &[],
    },
};
