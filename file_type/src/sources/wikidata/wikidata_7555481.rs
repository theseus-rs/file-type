use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_7555481: FileType = FileType {
    file_format: &FileFormat {
        id: 7_555_481,
        source_type: SourceType::Wikidata,
        name: "sol",
        extensions: &["sol"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
