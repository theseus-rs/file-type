use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28758111: FileType = FileType {
    file_format: &FileFormat {
        id: 28_758_111,
        source_type: SourceType::Wikidata,
        name: "Internet Explorer favorites",
        extensions: &["url"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
