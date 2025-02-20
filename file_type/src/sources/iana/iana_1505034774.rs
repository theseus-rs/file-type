use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1505034774: FileType = FileType {
    file_format: &FileFormat {
        id: 1_505_034_774,
        source_type: SourceType::Iana,
        name: "vnd.micrografx.flo",
        extensions: &[],
        media_types: &["application/vnd.micrografx.flo"],
        signatures: &[],
        related_formats: &[],
    },
};
