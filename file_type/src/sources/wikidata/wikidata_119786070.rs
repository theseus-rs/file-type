use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_119786070: FileType = FileType {
    file_format: &FileFormat {
        id: 119_786_070,
        source_type: SourceType::Wikidata,
        name: "MasterCook Tips File",
        extensions: &["mtf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
