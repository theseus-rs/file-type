use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3145090338: FileType = FileType {
    file_format: &FileFormat {
        id: 3_145_090_338,
        source_type: SourceType::Iana,
        name: "vnd.cups-raw",
        extensions: &[],
        media_types: &["application/vnd.cups-raw"],
        signatures: &[],
        related_formats: &[],
    },
};
