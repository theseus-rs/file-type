use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4233524847: FileType = FileType {
    file_format: &FileFormat {
        id: 4_233_524_847,
        source_type: SourceType::Iana,
        name: "prs.aimg",
        extensions: &[],
        media_types: &["image/prs.aimg"],
        signatures: &[],
        related_formats: &[],
    },
};
