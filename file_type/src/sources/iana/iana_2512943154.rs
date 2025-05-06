use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2512943154: FileType = FileType {
    file_format: &FileFormat {
        id: 2_512_943_154,
        source_type: SourceType::Iana,
        name: "vnd.typst",
        extensions: &[],
        media_types: &["text/vnd.typst"],
        signatures: &[],
        related_formats: &[],
    },
};
