use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3640433183: FileType = FileType {
    file_format: &FileFormat {
        id: 3_640_433_183,
        source_type: SourceType::Iana,
        name: "scvp-vp-request",
        extensions: &[],
        media_types: &["application/scvp-vp-request"],
        signatures: &[],
        related_formats: &[],
    },
};
