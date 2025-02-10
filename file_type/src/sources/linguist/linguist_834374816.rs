use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_834374816: FileType = FileType {
    file_format: &FileFormat {
        id: 834_374_816,
        source_type: SourceType::Linguist,
        name: "OASv2-json",
        extensions: &["json"],
        media_types: &["application/json"],
        signatures: &[],
        related_formats: &[],
    },
};
