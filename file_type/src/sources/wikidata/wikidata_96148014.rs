use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_96148014: FileType = FileType {
    file_format: &FileFormat {
        id: 96_148_014,
        source_type: SourceType::Wikidata,
        name: "Wolfram Language neural net format",
        extensions: &["wlnet"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
