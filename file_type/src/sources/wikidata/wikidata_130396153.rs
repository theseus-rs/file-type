use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130396153: FileType = FileType {
    file_format: &FileFormat {
        id: 130_396_153,
        source_type: SourceType::Wikidata,
        name: "Ooc source code file",
        extensions: &["ooc"],
        media_types: &["text/x-ooc"],
        signatures: &[],
        related_formats: &[],
    },
};
