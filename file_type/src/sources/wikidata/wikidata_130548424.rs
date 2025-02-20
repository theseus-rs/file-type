use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130548424: FileType = FileType {
    file_format: &FileFormat {
        id: 130_548_424,
        source_type: SourceType::Wikidata,
        name: "QBasic source code file",
        extensions: &["bas"],
        media_types: &["text/basic"],
        signatures: &[],
        related_formats: &[],
    },
};
