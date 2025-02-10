use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1224104050: FileType = FileType {
    file_format: &FileFormat {
        id: 1_224_104_050,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.5gsa2x",
        extensions: &[],
        media_types: &["application/vnd.3gpp.5gsa2x"],
        signatures: &[],
        related_formats: &[],
    },
};
