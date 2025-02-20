use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3735194806: FileType = FileType {
    file_format: &FileFormat {
        id: 3_735_194_806,
        source_type: SourceType::Iana,
        name: "vnd.sema",
        extensions: &[],
        media_types: &["application/vnd.sema"],
        signatures: &[],
        related_formats: &[],
    },
};
