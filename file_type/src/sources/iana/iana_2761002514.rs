use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2761002514: FileType = FileType {
    file_format: &FileFormat {
        id: 2_761_002_514,
        source_type: SourceType::Iana,
        name: "vnd.blockfact.facti",
        extensions: &[],
        media_types: &["image/vnd.blockfact.facti"],
        signatures: &[],
        related_formats: &[],
    },
};
