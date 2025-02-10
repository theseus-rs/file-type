use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2211731750: FileType = FileType {
    file_format: &FileFormat {
        id: 2_211_731_750,
        source_type: SourceType::Iana,
        name: "flexfec",
        extensions: &[],
        media_types: &["audio/flexfec"],
        signatures: &[],
        related_formats: &[],
    },
};
