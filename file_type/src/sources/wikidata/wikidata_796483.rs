use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_796483: FileType = FileType {
    file_format: &FileFormat {
        id: 796_483,
        source_type: SourceType::Wikidata,
        name: "DocBook",
        extensions: &["dbk", "xml"],
        media_types: &["application/docbook+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
