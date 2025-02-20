use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4012838875: FileType = FileType {
    file_format: &FileFormat {
        id: 4_012_838_875,
        source_type: SourceType::Iana,
        name: "vnd.denovo.fcselayout-link",
        extensions: &[],
        media_types: &["application/vnd.denovo.fcselayout-link"],
        signatures: &[],
        related_formats: &[],
    },
};
