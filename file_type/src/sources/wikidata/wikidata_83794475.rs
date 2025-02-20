use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_83794475: FileType = FileType {
    file_format: &FileFormat {
        id: 83_794_475,
        source_type: SourceType::Wikidata,
        name: "FO File",
        extensions: &["fo"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
