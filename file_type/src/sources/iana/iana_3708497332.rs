use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3708497332: FileType = FileType {
    file_format: &FileFormat {
        id: 3_708_497_332,
        source_type: SourceType::Iana,
        name: "srgs",
        extensions: &[],
        media_types: &["application/srgs"],
        signatures: &[],
        related_formats: &[],
    },
};
