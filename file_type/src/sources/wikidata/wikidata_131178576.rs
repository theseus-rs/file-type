use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_131178576: FileType = FileType {
    file_format: &FileFormat {
        id: 131_178_576,
        source_type: SourceType::Wikidata,
        name: "SWIG source code file",
        extensions: &["swg"],
        media_types: &["text/swig"],
        signatures: &[],
        related_formats: &[],
    },
};
