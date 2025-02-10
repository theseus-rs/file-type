use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4251501848: FileType = FileType {
    file_format: &FileFormat {
        id: 4_251_501_848,
        source_type: SourceType::Iana,
        name: "opc-nodeset+xml",
        extensions: &[],
        media_types: &["application/opc-nodeset+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
