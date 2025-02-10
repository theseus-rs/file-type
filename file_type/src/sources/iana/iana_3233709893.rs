use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3233709893: FileType = FileType {
    file_format: &FileFormat {
        id: 3_233_709_893,
        source_type: SourceType::Iana,
        name: "vnd.wap.slc",
        extensions: &[],
        media_types: &["application/vnd.wap.slc"],
        signatures: &[],
        related_formats: &[],
    },
};
