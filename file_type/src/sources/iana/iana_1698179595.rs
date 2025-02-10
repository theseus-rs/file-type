use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1698179595: FileType = FileType {
    file_format: &FileFormat {
        id: 1_698_179_595,
        source_type: SourceType::Iana,
        name: "route-usd+xml",
        extensions: &[],
        media_types: &["application/route-usd+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
