use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3025077792: FileType = FileType {
    file_format: &FileFormat {
        id: 3_025_077_792,
        source_type: SourceType::Iana,
        name: "global-disposition-notification",
        extensions: &[],
        media_types: &["message/global-disposition-notification"],
        signatures: &[],
        related_formats: &[],
    },
};
