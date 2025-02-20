use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3416960060: FileType = FileType {
    file_format: &FileFormat {
        id: 3_416_960_060,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.5gsv2x",
        extensions: &[],
        media_types: &["application/vnd.3gpp.5gsv2x"],
        signatures: &[],
        related_formats: &[],
    },
};
