use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1310312502: FileType = FileType {
    file_format: &FileFormat {
        id: 1_310_312_502,
        source_type: SourceType::Iana,
        name: "spdx3+json",
        extensions: &[],
        media_types: &["application/spdx3+json"],
        signatures: &[],
        related_formats: &[],
    },
};
