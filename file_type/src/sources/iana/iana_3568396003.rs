use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3568396003: FileType = FileType {
    file_format: &FileFormat {
        id: 3_568_396_003,
        source_type: SourceType::Iana,
        name: "vnd.coffeescript",
        extensions: &[],
        media_types: &["application/vnd.coffeescript"],
        signatures: &[],
        related_formats: &[],
    },
};
