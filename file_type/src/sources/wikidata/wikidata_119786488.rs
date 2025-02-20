use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_119786488: FileType = FileType {
    file_format: &FileFormat {
        id: 119_786_488,
        source_type: SourceType::Wikidata,
        name: "MasterCook Export File",
        extensions: &["mx2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
