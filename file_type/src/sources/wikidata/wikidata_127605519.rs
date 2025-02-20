use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_127605519: FileType = FileType {
    file_format: &FileFormat {
        id: 127_605_519,
        source_type: SourceType::Wikidata,
        name: "Crystal file format",
        extensions: &["cr"],
        media_types: &["text/x-crystal"],
        signatures: &[],
        related_formats: &[],
    },
};
