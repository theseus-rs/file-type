use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1182008171: FileType = FileType {
    file_format: &FileFormat {
        id: 1_182_008_171,
        source_type: SourceType::Iana,
        name: "vnd.novadigm.EDM",
        extensions: &[],
        media_types: &["application/vnd.novadigm.EDM"],
        signatures: &[],
        related_formats: &[],
    },
};
