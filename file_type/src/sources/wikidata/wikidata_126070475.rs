use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_126070475: FileType = FileType {
    file_format: &FileFormat {
        id: 126_070_475,
        source_type: SourceType::Wikidata,
        name: "Sibelius Scorch",
        extensions: &["sco"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
