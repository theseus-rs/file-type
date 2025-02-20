use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_131438617: FileType = FileType {
    file_format: &FileFormat {
        id: 131_438_617,
        source_type: SourceType::Wikidata,
        name: "Xtend file format",
        extensions: &["xtend"],
        media_types: &["text/x-xtend"],
        signatures: &[],
        related_formats: &[],
    },
};
