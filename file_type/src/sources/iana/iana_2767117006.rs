use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2767117006: FileType = FileType {
    file_format: &FileFormat {
        id: 2_767_117_006,
        source_type: SourceType::Iana,
        name: "ohttp-chunked-res",
        extensions: &[],
        media_types: &["message/ohttp-chunked-res"],
        signatures: &[],
        related_formats: &[],
    },
};
