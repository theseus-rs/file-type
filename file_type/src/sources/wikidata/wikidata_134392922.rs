use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_134392922: FileType = FileType {
    file_format: &FileFormat {
        id: 134_392_922,
        source_type: SourceType::Wikidata,
        name: "Q134392922",
        extensions: &["watchface"],
        media_types: &["application/vnd.apple.watchface"],
        signatures: &[],
        related_formats: &[],
    },
};
