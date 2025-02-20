use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2229859570: FileType = FileType {
    file_format: &FileFormat {
        id: 2_229_859_570,
        source_type: SourceType::Iana,
        name: "sensml+xml",
        extensions: &[],
        media_types: &["application/sensml+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
