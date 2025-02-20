use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2220125132: FileType = FileType {
    file_format: &FileFormat {
        id: 2_220_125_132,
        source_type: SourceType::Iana,
        name: "vnd.xmpie.plan",
        extensions: &[],
        media_types: &["application/vnd.xmpie.plan"],
        signatures: &[],
        related_formats: &[],
    },
};
