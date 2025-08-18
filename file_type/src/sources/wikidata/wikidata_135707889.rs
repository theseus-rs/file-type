use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_135707889: FileType = FileType {
    file_format: &FileFormat {
        id: 135_707_889,
        source_type: SourceType::Wikidata,
        name: "occam source code file",
        extensions: &["occ"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
