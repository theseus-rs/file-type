use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_258778: FileType = FileType {
    file_format: &FileFormat {
        id: 258_778,
        source_type: SourceType::Wikidata,
        name: "Extensible Application Markup Language",
        extensions: &["xaml"],
        media_types: &["application/xaml+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
