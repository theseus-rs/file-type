use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_896475446: FileType = FileType {
    file_format: &FileFormat {
        id: 896_475_446,
        source_type: SourceType::Iana,
        name: "vnd.collabio.xodocuments.presentation",
        extensions: &[],
        media_types: &["application/vnd.collabio.xodocuments.presentation"],
        signatures: &[],
        related_formats: &[],
    },
};
