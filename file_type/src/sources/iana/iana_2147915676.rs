use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2147915676: FileType = FileType {
    file_format: &FileFormat {
        id: 2_147_915_676,
        source_type: SourceType::Iana,
        name: "vnd.xmpie.xlim",
        extensions: &[],
        media_types: &["application/vnd.xmpie.xlim"],
        signatures: &[],
        related_formats: &[],
    },
};
