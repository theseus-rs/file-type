use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_647269280: FileType = FileType {
    file_format: &FileFormat {
        id: 647_269_280,
        source_type: SourceType::Iana,
        name: "mpeg4-iod-xmt",
        extensions: &[],
        media_types: &["application/mpeg4-iod-xmt"],
        signatures: &[],
        related_formats: &[],
    },
};
