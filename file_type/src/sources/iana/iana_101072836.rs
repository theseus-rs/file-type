use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_101072836: FileType = FileType {
    file_format: &FileFormat {
        id: 101_072_836,
        source_type: SourceType::Iana,
        name: "vnd.syncml.ds.notification",
        extensions: &[],
        media_types: &["application/vnd.syncml.ds.notification"],
        signatures: &[],
        related_formats: &[],
    },
};
