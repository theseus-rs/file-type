use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1874272716: FileType = FileType {
    file_format: &FileFormat {
        id: 1_874_272_716,
        source_type: SourceType::Iana,
        name: "mpeg4-generic",
        extensions: &[],
        media_types: &["audio/mpeg4-generic"],
        signatures: &[],
        related_formats: &[],
    },
};
