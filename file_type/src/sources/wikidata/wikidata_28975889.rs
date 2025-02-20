use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28975889: FileType = FileType {
    file_format: &FileFormat {
        id: 28_975_889,
        source_type: SourceType::Wikidata,
        name: "Potential Control File",
        extensions: &["pot"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
