use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2577903933: FileType = FileType {
    file_format: &FileFormat {
        id: 2_577_903_933,
        source_type: SourceType::Iana,
        name: "vc+sd-jwt",
        extensions: &[],
        media_types: &["application/vc+sd-jwt"],
        signatures: &[],
        related_formats: &[],
    },
};
