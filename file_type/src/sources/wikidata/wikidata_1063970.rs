use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1063970: FileType = FileType {
    file_format: &FileFormat {
        id: 1_063_970,
        source_type: SourceType::Wikidata,
        name: "VP8",
        extensions: &[],
        media_types: &["video/VP8"],
        signatures: &[],
        related_formats: &[],
    },
};
