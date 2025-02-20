use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125858086: FileType = FileType {
    file_format: &FileFormat {
        id: 125_858_086,
        source_type: SourceType::Wikidata,
        name: "Python GUI Source File",
        extensions: &["pyw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
