use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_127605323: FileType = FileType {
    file_format: &FileFormat {
        id: 127_605_323,
        source_type: SourceType::Wikidata,
        name: "Ceylon source code file",
        extensions: &["ceylon"],
        media_types: &["text/x-ceylon"],
        signatures: &[],
        related_formats: &[],
    },
};
