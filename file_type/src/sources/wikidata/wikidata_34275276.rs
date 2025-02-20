use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_34275276: FileType = FileType {
    file_format: &FileFormat {
        id: 34_275_276,
        source_type: SourceType::Wikidata,
        name: "Numbers Zipped",
        extensions: &["numbers.zip"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
