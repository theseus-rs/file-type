use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_109265753: FileType = FileType {
    file_format: &FileFormat {
        id: 109_265_753,
        source_type: SourceType::Wikidata,
        name: "PagePlus Template",
        extensions: &["ppt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
