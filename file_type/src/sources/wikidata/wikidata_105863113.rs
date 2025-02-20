use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_105863113: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_113,
        source_type: SourceType::Wikidata,
        name: "mzXML",
        extensions: &["mzXML"],
        media_types: &["text/xml"],
        signatures: &[],
        related_formats: &[],
    },
};
