use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2735872475: FileType = FileType {
    file_format: &FileFormat {
        id: 2_735_872_475,
        source_type: SourceType::Iana,
        name: "vnd.jam",
        extensions: &[],
        media_types: &["application/vnd.jam"],
        signatures: &[],
        related_formats: &[],
    },
};
