use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3296732317: FileType = FileType {
    file_format: &FileFormat {
        id: 3_296_732_317,
        source_type: SourceType::Iana,
        name: "spdx",
        extensions: &[],
        media_types: &["text/spdx"],
        signatures: &[],
        related_formats: &[],
    },
};
