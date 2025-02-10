use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130284538: FileType = FileType {
    file_format: &FileFormat {
        id: 130_284_538,
        source_type: SourceType::Wikidata,
        name: "MCfunction script file",
        extensions: &["mcfunction"],
        media_types: &["text/mcfunction"],
        signatures: &[],
        related_formats: &[],
    },
};
