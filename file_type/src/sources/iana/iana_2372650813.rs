use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2372650813: FileType = FileType {
    file_format: &FileFormat {
        id: 2_372_650_813,
        source_type: SourceType::Iana,
        name: "cdmi-capability",
        extensions: &[],
        media_types: &["application/cdmi-capability"],
        signatures: &[],
        related_formats: &[],
    },
};
