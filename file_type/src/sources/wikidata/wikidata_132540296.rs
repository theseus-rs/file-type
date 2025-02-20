use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_132540296: FileType = FileType {
    file_format: &FileFormat {
        id: 132_540_296,
        source_type: SourceType::Wikidata,
        name: "Discrete Fracture Properties file format",
        extensions: &["fprops"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
