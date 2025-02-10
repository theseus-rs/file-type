use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_152912554: FileType = FileType {
    file_format: &FileFormat {
        id: 152_912_554,
        source_type: SourceType::Iana,
        name: "vnd.semf",
        extensions: &[],
        media_types: &["application/vnd.semf"],
        signatures: &[],
        related_formats: &[],
    },
};
