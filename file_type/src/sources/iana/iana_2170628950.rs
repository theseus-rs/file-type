use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2170628950: FileType = FileType {
    file_format: &FileFormat {
        id: 2_170_628_950,
        source_type: SourceType::Iana,
        name: "vnd.usda",
        extensions: &[],
        media_types: &["model/vnd.usda"],
        signatures: &[],
        related_formats: &[],
    },
};
