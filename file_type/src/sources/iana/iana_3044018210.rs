use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3044018210: FileType = FileType {
    file_format: &FileFormat {
        id: 3_044_018_210,
        source_type: SourceType::Iana,
        name: "vnd.softpres-ipf-disk-image",
        extensions: &[],
        media_types: &["application/vnd.softpres-ipf-disk-image"],
        signatures: &[],
        related_formats: &[],
    },
};
