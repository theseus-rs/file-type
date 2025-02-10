use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_489923641: FileType = FileType {
    file_format: &FileFormat {
        id: 489_923_641,
        source_type: SourceType::Iana,
        name: "vnd.syncml.dmddf+wbxml",
        extensions: &[],
        media_types: &["application/vnd.syncml.dmddf+wbxml"],
        signatures: &[],
        related_formats: &[],
    },
};
