use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1893947460: FileType = FileType {
    file_format: &FileFormat {
        id: 1_893_947_460,
        source_type: SourceType::Iana,
        name: "cose-key",
        extensions: &[],
        media_types: &["application/cose-key"],
        signatures: &[],
        related_formats: &[],
    },
};
