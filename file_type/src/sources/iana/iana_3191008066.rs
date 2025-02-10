use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3191008066: FileType = FileType {
    file_format: &FileFormat {
        id: 3_191_008_066,
        source_type: SourceType::Iana,
        name: "vnd.shana.informed.formdata",
        extensions: &[],
        media_types: &["application/vnd.shana.informed.formdata"],
        signatures: &[],
        related_formats: &[],
    },
};
