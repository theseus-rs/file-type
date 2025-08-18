use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1312241452: FileType = FileType {
    file_format: &FileFormat {
        id: 1_312_241_452,
        source_type: SourceType::Iana,
        name: "vnd.R74n.sandboxels+json",
        extensions: &[],
        media_types: &["application/vnd.R74n.sandboxels+json"],
        signatures: &[],
        related_formats: &[],
    },
};
