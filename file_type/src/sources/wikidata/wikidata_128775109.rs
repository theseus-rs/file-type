use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_128775109: FileType = FileType {
    file_format: &FileFormat {
        id: 128_775_109,
        source_type: SourceType::Wikidata,
        name: "Component Pascal source code file",
        extensions: &["cp"],
        media_types: &["text/x-component-pascal"],
        signatures: &[],
        related_formats: &[],
    },
};
